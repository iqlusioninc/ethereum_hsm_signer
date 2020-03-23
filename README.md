# EthereumSigner

EthereumSigner is application for use Signatory and it's various backends especially the yubihsm to sign ethereum transactions. It talks to a grpc server whose api is defined in `proto/SigService.proto`.

There are two methods.

```
    rpc GetSignable(Empty) returns (Signable) {}
    rpc ProvideSig(Signed) returns (Empty) {}
```

One gets a messages to sign and the other provides the message back with signature.



## Getting Started

This application is authored using [Abscissa], a Rust application framework.

For more information, see:

[Documentation]

[Abscissa]: https://github.com/iqlusioninc/abscissa
[Documentation]: https://docs.rs/abscissa_core/
