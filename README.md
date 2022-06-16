# Wdes Mail AutoDiscover-AutoConfig

## Support

- Thunderbird
- Microsoft
- Apple (kind of)

## Use it

Pull the [image](https://hub.docker.com/r/wdes/mail-autodiscover-autoconfig): `docker pull wdes/mail-autodiscover-autoconfig`

Save this `docker-compose.yml` file and adjust it for your needs.

```yml
version: "3"

services:
    mailserver:
        image: wdes/mail-autodiscover-autoconfig:latest
        mem_limit: 120M
        mem_reservation: 50M
        restart: on-failure:40
        ports:
            - "8088:80"
        environment:
            ROCKET_PROFILE: production
            ROCKET_ADDRESS: "0.0.0.0"
            ROCKET_PORT: "80"
            # https://www.uuidgenerator.net/
            # Re-generate the two UUIDs below
            APPLE_MAIL_UUID: ff83a13b-c4e6-41c5-bf54-0a244bb3bf5d
            APPLE_PROFILE_UUID: b8e39daa-64a3-4928-bb86-cb4b551fdd57
            # List of domains that will use {imap,pop,smtp}.domain.tld instead of the hosts below
            CUSTOM_DOMAINS: foo.tld
            IMAP_HOSTNAME: imap.mails.provider.tld
            POP_HOSTNAME: pop.mails.provider.tld
            SMTP_HOSTNAME: smtp.mails.provider.tld
```
