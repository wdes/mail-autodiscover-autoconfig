# Wdes Mail AutoDiscover-AutoConfig

## Support

- Thunderbird
- Microsoft Outlook
- Apple MacOS
- Apple iOS

## Inspiration

- [mailcow's autodiscover-json.php](https://github.com/mailcow/mailcow-dockerized/blob/master/data/web/autodiscover-json.php)
- [Monogramm's autodiscover-email-settings Node project](https://github.com/Monogramm/autodiscover-email-settings)
- [Wicloz's EmailAutoConfig templates](https://github.com/Wicloz/EmailAutoConfig/tree/8e02dbd6dca7edfd748802028ba711289a7fe1a5/templates)
- [Mailu's logic for AcceptableResponseSchema](https://github.com/Mailu/Mailu/blob/c15e4e6015592735fa6f730af72b8332e93ae672/core/admin/mailu/internal/views/autoconfig.py#L55-L91)
- [hmlkao's php-autodiscover for Apps to test](https://github.com/hmlkao/php-autodiscover#readme)
- [StephanvanSchaik's automail project written in Rust](https://github.com/StephanvanSchaik/automail)

## Tested working apps

- [Apple MacOS Monterey](https://www.apple.com/fr/macos/monterey/) on Apple Mail (MacOS: 12.2, Mail: 15.0 (3693.60.0.1.1))
- [Thunderbird](https://www.thunderbird.net/) (tested version: 91.10.0)
- [Evolution on Ubuntu](https://wiki.gnome.org/Apps/Evolution/) (tested version: [3.40.0](https://gitlab.gnome.org/GNOME/evolution/-/tree/3.40.0/))
- [KMail on Ubuntu](https://userbase.kde.org/KMail) (tested version: 5.19.3 (21.12.3))
- [Microsoft Office Pro Plus 2013](https://wikipedia.org/wiki/Microsoft_Office_2013) (tested version: 15.0.5399.1000 64 bits)
- <!: [You need to disable Office 365 setup and adjust the config manually](https://github.com/smartlyway/email-autoconfig-php/issues/2)> [Microsoft Office Pro Plus 2021](https://wikipedia.org/wiki/Microsoft_Office_2013) (tested version: 14326.204454 64 bits)

### Phone Apps

- [Apple iOS 15.6.1](https://support.apple.com/en-us/HT213412) (iOS: 15.6.1)
- [FairEmail](https://github.com/M66B/FairEmail) (tested version: 1.1917)
- [Android Nine](https://www.9folders.com/en/index.html) (tested version: [4.9.4b](https://play.google.com/store/apps/details?id=com.ninefolders.hd3))
- [Spark Mail on Android](https://sparkmailapp.com/) (tested version: 2.11.8)
- [MailTime on Android](https://mailtime.com/) (tested version: 2.5.4.0614)
- [Maily on Android](https://github.com/Enough-Software/enough_mail_app#readme) (tested version: [1.0.0](https://play.google.com/store/apps/details?id=de.enough.enough_mail_app))

## Use it

Pull the [image](https://hub.docker.com/r/wdes/mail-autodiscover-autoconfig): `docker pull wdes/mail-autodiscover-autoconfig`

Save this `docker-compose.yml` file and adjust it for your needs.

```yml
version: "3"

services:
    mail-autodiscover-autoconfig:
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
            # CUSTOM_DOMAINS: foo.tld
            # (optional: CUSTOM_DOMAINS) List of domains that will use {imap,pop,smtp}.domain.tld instead of the hosts below
            IMAP_HOSTNAME: imap.mails.provider.tld
            POP_HOSTNAME: pop.mails.provider.tld
            SMTP_HOSTNAME: smtp.mails.provider.tld
```

### Install on Apple

- Browse `https://autoconfig.<domain>.<tld>/apple` or directly `https://autoconfig.<domain>.<tld>/email.mobileconfig?email=<username>@<domain>.<tld>`
- Fill and submit the form, the profile downloads itself
- Click on it, it says to go to system settings to enable it
- Go to system settings, then "Profiles" or "Downloaded Profiles"
- Click install on the profile, enter your password
- You can go back to your Mail app, the email configured

NB: Is Apple signing implemented: No. Will it be: maybe, if someone/I figures it out ^^

### Technical compilation aspects

You can disable some of the default features. For now the default features are:

- `apple` (This feature contains the web UI route and the mobile config route)
- `dns` (This feature adds a DNS txt route to obtain your ideal DNS config)
