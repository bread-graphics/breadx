#               Copyright John Nunley, 2022.
# Distributed under the Boost Software License, Version 1.0.
#       (See accompanying file LICENSE or copy at
#         https://www.boost.org/LICENSE_1_0.txt)

protocol:
    cargo run --manifest-path breadx-generator/Cargo.toml \
        xcbproto/src \
        breadx/src/automatically_generated.rs