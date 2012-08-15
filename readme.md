##About

This is a really simple wrapper around code from libsyntax. It basically takes the minimal extraction from what rustc --pretty normal does, and puts it in it's own binary. This is a hack. Hopefully it is a useful one :)

##Installing

You should be able to install this from cargo, using `cargo install rustfmt`.

##Working on this / Improving

If you want this formatter to be better, you need to work on the pretty printer is libsyntax/print in the main rust repository. The code for this package should not have to change, unless the overall structure of parsing / printing changes within the compiler.