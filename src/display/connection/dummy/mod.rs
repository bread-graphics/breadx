// MIT/Apache2 License

mod setup;

use crate::{auto::xproto::QueryExtensionReply, auto::AsByteSequence, Connection, Fd, Request};
use alloc::vec::Vec;
use core::{array::IntoIter as ArrayIter, cmp, iter};
use std::collections::HashMap;

/// Expected transaction connection.
///
/// This connection operates on a list of expected "transactions" - sending and receiving data. Data that the
/// connection receives (i.e. the client sends) will be tested and will result in panic, while data that the
/// connection sends (i.e. the client receives) will be preprogrammed.
pub struct PreprogrammedConnection<I> {
    transactions: I,
    current: Option<Transaction>,
    sequence: u16,
    extensions: HashMap<&'static str, u8>,
}

impl<I: Iterator<Item = Transaction>> PreprogrammedConnection<I> {
    #[inline]
    fn new<II: IntoIterator<IntoIter = I>>(iter: II) -> PreprogrammedConnection<I> {
        PreprogrammedConnection {
            current: None,
            transactions: iter.into_iter(),
            sequence: 1,
            extensions: HashMap::new(),
        }
    }

    #[inline]
    fn transaction(&mut self, debug_bytes: &[u8]) -> Transaction {
        match self.current.take() {
            Some(current) => current,
            None => self.transactions.next().unwrap_or_else(|| {
                panic!(
                    "Preprogrammed connection ran out of transactions while processing bytes: {:?}",
                    debug_bytes
                )
            }),
        }
    }

    #[inline]
    fn store_transaction(&mut self, txn: Transaction) {
        self.current = Some(txn);
    }

    #[inline]
    pub fn use_extension(&mut self, name: &'static str, value: u8) {
        self.extensions.insert(name, value);
    }
}

const TRANSACTION_COUNT: usize = 4;

impl<I: Iterator<Item = Transaction>>
    PreprogrammedConnection<iter::Chain<ArrayIter<Transaction, 4>, I>>
{
    /// Responds to a `breadx` connection's attempt to connect to a fictional server.
    #[inline]
    pub fn normal_setup<II: IntoIterator<IntoIter = I>>(iter: II) -> Self {
        let denied = QueryExtensionReply {
            reply_type: 1,
            sequence: 1,
            length: 0,
            present: false,
            ..Default::default()
        };

        let nsetup: [Transaction; TRANSACTION_COUNT] = [
            // setup request
            Transaction::unchecked_sends(None),
            // setup reply
            Transaction::receives(setup::dummy_setup()),
            // bigreq request
            Transaction::unchecked_sends(None),
            // tell them we don't have bigreq
            Transaction::receives(denied),
        ];

        let mut this = Self::new(ArrayIter::new(nsetup).chain(iter.into_iter()));
        this.sequence += 1;
        this
    }
}

impl<I: Iterator<Item = Transaction>> Connection for PreprogrammedConnection<I> {
    #[inline]
    fn send_packet(&mut self, bytes: &[u8], _fds: &mut Vec<Fd>) -> crate::Result {
        log::info!(
            "Preprogrammed connection receives {} bytes of data",
            bytes.len()
        );

        if bytes.is_empty() {
            return Ok(());
        }

        let mut txn = self.transaction(bytes);

        match txn.ty {
            TransactionType::ClientSendsData => {
                assert_eq!(&txn.data, bytes, "Client sent non-matching data")
            }
            TransactionType::ClientSendsUncheckedData {
                expected_len: Some(len),
            } => assert_eq!(bytes.len(), len, "Client sent non-matching length"),
            TransactionType::ClientSendsUncheckedData { expected_len: None } => {}
            TransactionType::ClientSendsRequest { opcode, extension } => {
                if let Some(extension) = extension {
                    let extcode = *self
                        .extensions
                        .get(extension)
                        .expect("Extension not logged");
                    txn.data[0] = extcode;
                    txn.data[1] = opcode;
                } else {
                    txn.data[0] = opcode;
                }

                assert_eq!(
                    &txn.data[..2],
                    &bytes[..2],
                    "Client sent non-matching request header"
                );
                assert_eq!(
                    &txn.data[4..],
                    &bytes[4..],
                    "Client sent non-matching request data"
                )
            }
            TransactionType::ClientReceivesData | TransactionType::ClientReceivesReply => {
                panic!("Client sent data when it was supposed to receive")
            }
        }

        Ok(())
    }

    #[inline]
    fn read_packet(&mut self, bytes: &mut [u8], _fds: &mut Vec<Fd>) -> crate::Result {
        log::info!(
            "Preprogrammed connection is expected to fill {} bytes of data",
            bytes.len()
        );

        if bytes.is_empty() {
            return Ok(());
        }

        let mut txn = self.transaction(&[]);

        match txn.ty {
            TransactionType::ClientReceivesData => {}
            TransactionType::ClientReceivesReply => {
                let seq = self.sequence;
                self.sequence = self.sequence.wrapping_add(1);
                txn.data[0] = 1;
                txn.ty = TransactionType::ClientReceivesData;
                (&mut txn.data[2..4]).copy_from_slice(&seq.to_ne_bytes());
            }
            _ => panic!("Client listened for data when it was supposed to send"),
        }

        let take = cmp::min(txn.data.len(), bytes.len());
        (&mut bytes[..take]).copy_from_slice(&txn.data[..take]);

        if take >= txn.data.len() {
            // need a new transaction
        } else {
            // cut off transaction data that's already read
            let leftovers = txn.data.split_off(take);
            txn.data = leftovers;
            self.store_transaction(txn);
        }

        Ok(())
    }
}

/// A preprogrammed transaction between the client and the server.
pub struct Transaction {
    data: Vec<u8>,
    ty: TransactionType,
}

#[derive(Clone, Copy)]
enum TransactionType {
    ClientSendsData,
    ClientReceivesData,
    ClientSendsUncheckedData {
        expected_len: Option<usize>,
    },
    ClientSendsRequest {
        opcode: u8,
        extension: Option<&'static str>,
    },
    ClientReceivesReply,
}

impl Transaction {
    #[inline]
    pub fn unchecked_sends(expected_len: Option<usize>) -> Transaction {
        Transaction {
            data: Vec::new(),
            ty: TransactionType::ClientSendsUncheckedData { expected_len },
        }
    }

    #[inline]
    pub fn receives<T: AsByteSequence>(obj: T) -> Transaction {
        Transaction::from_data(obj, TransactionType::ClientReceivesData)
    }

    #[inline]
    pub fn sends<T: AsByteSequence>(obj: T) -> Transaction {
        Transaction::from_data(obj, TransactionType::ClientSendsData)
    }

    #[inline]
    pub fn request<R: Request>(req: R) -> Transaction {
        let mut data = iter::repeat(0).take(req.size()).collect::<Vec<u8>>();
        req.as_bytes(&mut data);
        if R::EXTENSION.is_none() {
            data[0] = R::OPCODE;
        } else {
            data[1] = R::OPCODE;
        }
        Transaction {
            data,
            ty: TransactionType::ClientSendsRequest {
                opcode: R::OPCODE,
                extension: R::EXTENSION,
            },
        }
    }

    #[inline]
    pub fn reply<T: AsByteSequence>(obj: T) -> Transaction {
        Transaction::from_data(obj, TransactionType::ClientReceivesReply)
    }

    #[inline]
    fn from_data<T: AsByteSequence>(obj: T, ty: TransactionType) -> Transaction {
        let mut data = iter::repeat(0).take(obj.size()).collect::<Vec<u8>>();
        obj.as_bytes(&mut data);
        Transaction { data, ty }
    }
}
