# Running the Node in a Container (Preferred Method)

### Requirements

* TLS Certificates for your Node
* Node Signer Keys (See Signing Consensus Messages) \[add bookmark]&#x20;
* Network Configuration File/Consensus config file (network.toml - See Configuring your Node to Connect to Trusted and Untrusted Peers) \[add bookmark]
* Ledger Storage Location
* S3 Storage Location

### Entrypoint and Container Processes

Familiarize yourself with the entrypoint for the consensus docker container. \[add bookmark]

{% hint style="info" %}
It contains multiple processes working together to provide the full Consensus Validator functionality.
{% endhint %}

These processes are the following:

| Process                 | Function                                                 |
| ----------------------- | -------------------------------------------------------- |
| `aesm_service`          | Provides EPID provisioning for the enclave.              |
| `filebeat`              | Provides logging.                                        |
| `mc-ledger-migration`   | Performs the ledger migration, if necessary, then stops. |
| `ledger-distribution`   | Distributes the ledger to S3 archive**.**                |
| `mc-admin-http-gateway` | Provides the admin panel to the Consensus Service.       |
| `consensus-service`     | Runs the MobileCoin Consensus Service.                   |
