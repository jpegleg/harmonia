# harmonia 🐞

Harmonia (see the insect Harmonia Axyridis) is a C2 server that runs on a high range TCP port bound with rusTLS.

This code is for educational purposes. While it does not use "unsafe" rust, because it allows passing of URI to sh there is plenty of potential for resource consumption that results in OOM etc.

Harmonia might be run in /var/tmp, or any location on the file system. It requires privkey.pem and cert.pem in the working directory. A common malicious pattern might be to copy the valid key and certificate and use those for the TLS identity. 

This malware works well as a statically linked binary. Once the malicious actor has gained access to the system, harmonia could be deployed to that system to enable remote code execution without SSH, etc.

There is an obfuscated URI context that is how the commands are inserted. Edit the context to your liking before compiling. The default port is 51472, feel free to adjust that as well. 

