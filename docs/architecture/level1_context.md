# Level 1: System Context

This guide is the 10K foot view of the system. The focus is on showing Automata Games and how it, as a system, interacts with other systems.

![AG Level 1 Architecture](../diagrams/out/context/ag_level1_overview.svg)

## [Name Cheap]
NameCheap is our DNS provider. This is where `automata.games`, our <abbr title="Top level Domain">TLD</abbr>, is registered.

## [Heroku]
Heroku is our cloud provider. It is where we deploy and run our application + managed resources our application needs like databases.

## [SendGrid]
SendGrid is our email as a service provider. Other services can be used like MailGun, but SendGrid has a good low-level free tier which is best for us at this stage. Plus, it's owned by Twilio, the dominant provider of text messaging as a service, which is pretty good, if we want to expand to SMS in the future.

## [Back to Architecture](../architecture.md)

[Name Cheap]:https://www.namecheap.com/
[Heroku]:https://dashboard.heroku.com/apps/automata-games
[SendGrid]:https://app.sendgrid.com/guide