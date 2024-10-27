# Fun with ktls

A relatively new addition to the Linux kernel is KTLS, which allows offloading of
the TLS encryption/decryption to operate in the kernel.

The benefit to doing this is generally to support usage of the `sendfile` syscall
directly over TLS, e.g. for a [file server that wants to serve files over HTTPS zero-copy](https://www.f5.com/company/blog/nginx/improving-nginx-performance-with-kernel-tls),
as well as enabling eBPF programs to run over the connection plaintext, or to enable
offloading to HW like the Nvidia BlueField.

## toys

There are two binaries, both using `rustls`. The first binary, `tls` just connections to a server and does
an HTTP request over userspace TLS.

The `ktls` binary does the same thing, but over a kernel-managed TLS connection instead.

## TODO

- [ ] Add example server implementations with `sendfile` for userspace/ktls
- [ ] Add Actions to Cross-compile for different targets
