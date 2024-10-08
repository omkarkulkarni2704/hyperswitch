 <h1 <p align="center">
  <a href="https://hyperswitch.io/" target="_blank"> hyperswitch </a>
 </p>
 </h1>
 <p align="center">
  <img src="./docs/imgs/hyperswitch-logo-dark.svg#gh-dark-mode-only" alt="Hyperswitch-Logo" width="40%" />
  <img src="./docs/imgs/hyperswitch-logo-light.svg#gh-light-mode-only" alt="Hyperswitch-Logo" width="40%" />
</p>

<h1 align="center">The open-source payments switch</h1>

<div align="center" >
The single API to access payment ecosystems across 130+ countries</div>

<p align="center">
  <a href="#try-a-payment">Try a Payment</a> •
  <a href="#for-enterprises">For Enterprises</a> •
  <a href="#for-contributors">For Contributors</a> •
  <a href="#quick-setup">Quick Setup</a> •
  <a href="/docs/try_local_system.md">Local Setup Guide (Hyperswitch App Server)</a> •
  <a href="#fast-integration-for-stripe-users">Fast Integration for Stripe Users</a> •
  <a href="https://api-reference.hyperswitch.io/introduction"> API Docs </a> 
   <br>
  <a href="#supported-features">Supported Features</a> •
  <a href="#community">Community</a> •
  <a href="#bugs-and-feature-requests">Bugs and feature requests</a> •
  <a href="#versioning">Versioning</a> •
  <a href="#FAQs">FAQs</a> •
  <a href="#copyright-and-license">Copyright and License</a>
</p>

<p align="center">
  <a href="https://github.com/juspay/hyperswitch/actions?query=workflow%3ACI+branch%3Amain">
    <img src="https://github.com/juspay/hyperswitch/workflows/CI/badge.svg" />
  </a>
  <a href="https://github.com/juspay/hyperswitch/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/juspay/hyperswitch" />
  </a>
  <a href="https://github.com/juspay/hyperswitch/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/Made_in-Rust-orange" />
  </a>
</p>
<p align="center">
  <a href="https://www.linkedin.com/company/hyperswitch/">
    <img src="https://img.shields.io/badge/follow-hyperswitch-blue?logo=linkedin&labelColor=grey"/>
  </a>
  <a href="https://x.com/hyperswitchio">
    <img src="https://img.shields.io/badge/follow-%40hyperswitchio-white?logo=x&labelColor=grey"/>
  </a>
  <a href="https://join.slack.com/t/hyperswitch-io/shared_invite/zt-2jqxmpsbm-WXUENx022HjNEy~Ark7Orw">
    <img src="https://img.shields.io/badge/chat-on_slack-blue?logo=slack&labelColor=grey&color=%233f0e40"/>
  </a>
</p>

<hr>
<img src="./docs/imgs/switch.png" />

Hyperswitch is a community-led, open payments switch to enable access to the best payments infrastructure for every digital business.

Using Hyperswitch, you can:

- ⬇️ **Reduce dependency** on a single processor like Stripe or Braintree
- 🧑‍💻 **Reduce Dev effort** by 90% to add & maintain integrations
- 🚀 **Improve success rates** with seamless failover and auto-retries
- 💸 **Reduce processing fees** with smart routing
- 🎨 **Customize payment flows** with full visibility and control
- 🌐 **Increase business reach** with local/alternate payment methods

<br>
<img src="./docs/imgs/hyperswitch-product.png" alt="Hyperswitch-Product" width="50%"/>

<a href="https://app.hyperswitch.io/">
  <h2 id="try-a-payment">⚡️ Try a Payment</h2>
</a>

To quickly experience the ease that Hyperswitch provides while handling the payment, you can signup on [hyperswitch-control-center][dashboard-link], and try a payment.

Congratulations 🎉 on making your first payment with Hyperswitch. 

<a href="#Get Started with Hyperswitch">
  <h2 id="get-started-with-hyperswitch">Get Started with Hyperswitch</h2>
</a>

### [For Enterprises][docs-link-for-enterprise]
  Hyperswitch helps enterprises in -  
  - Improving profitability
  - Increasing conversion rates
  - Lowering payment costs
  - Streamlining payment operations 
  
  Hyperswitch has ample features for businesses of all domains and sizes. [**Check out our offerings**][website-link]. 

### [For Contributors][contributing-guidelines]
  
  Hyperswitch is an open-source project that aims to make digital payments accessible to people across the globe like a basic utility. With the vision of developing Hyperswitch as the **Linux of Payments**, we seek support from developers worldwide.

  Utilise the following resources to quickstart your journey with Hyperswitch -
  - [Guide for contributors][contributing-guidelines]
  - [Developer Docs][docs-link-for-developers]
  - [Learning Resources][learning-resources]

<a href="#Quick Setup">
  <h2 id="quick-setup">⚡️ Quick Setup</h2>
</a>

### One-click deployment on AWS cloud

The fastest and easiest way to try Hyperswitch is via our CDK scripts

1. Click on the following button for a quick standalone deployment on AWS, suitable for prototyping.
   No code or setup is required in your system and the deployment is covered within the AWS free-tier setup.

   <a href="https://console.aws.amazon.com/cloudformation/home?region=us-east-1#/stacks/new?stackName=HyperswitchBootstarp&templateURL=https://hyperswitch-synth.s3.eu-central-1.amazonaws.com/hs-starter-config.yaml"><img src="./docs/imgs/aws_button.png" height="35"></a>

2. Sign-in to your AWS console.

3. Follow the instructions provided on the console to successfully deploy Hyperswitch

### Run it on your system

You can run Hyperswitch on your system using Docker Compose after cloning this repository:

```shell
git clone --depth 1 --branch latest https://github.com/juspay/hyperswitch
cd hyperswitch
docker compose up -d
```

This will start the app server, web client and control center.

Check out the [local setup guide][local-setup-guide] for a more comprehensive
setup, which includes the [scheduler and monitoring services][docker-compose-scheduler-monitoring].

[docs-link-for-enterprise]: https://docs.hyperswitch.io/hyperswitch-cloud/quickstart
[docs-link-for-developers]: https://docs.hyperswitch.io/hyperswitch-open-source/overview
[contributing-guidelines]: docs/CONTRIBUTING.md
[dashboard-link]: https://app.hyperswitch.io/
[website-link]: https://hyperswitch.io/
[learning-resources]: https://docs.hyperswitch.io/learn-more/payment-flows
[local-setup-guide]: /docs/try_local_system.md
[docker-compose-scheduler-monitoring]: /docs/try_local_system.md#running-additional-services
<a href="#Fast-Integration-for-Stripe-Users">
  <h2 id="fast-integration-for-stripe-users">🔌 Fast Integration for Stripe Users</h2>
</a>

If you are already using Stripe, integrating with Hyperswitch is fun, fast & easy.
Try the steps below to get a feel for how quick the setup is:

1. Get API keys from our [dashboard].
2. Follow the instructions detailed on our
   [documentation page][migrate-from-stripe].

[dashboard]: https://app.hyperswitch.io/register
[migrate-from-stripe]: https://hyperswitch.io/docs/migrateFromStripe

<a href="#Supported-Features">
  <h2 id="supported-features">✅ Supported Features</h2>
</a>

### 🌟 Supported Payment Processors and Methods

As of Aug 2024, Hyperswitch supports 50+ payment processors and multiple global payment methods.
In addition, we are continuously integrating new processors based on their reach and community requests.
Our target is to support 100+ processors by H2 2024.
You can find the latest list of payment processors, supported methods, and features [here][supported-connectors-and-features].

[supported-connectors-and-features]: https://hyperswitch.io/pm-list

### 🌟 Hosted Version

In addition to all the features of the open-source product, our hosted version
provides features and support to manage your payment infrastructure, compliance,
analytics, and operations end-to-end:

- **System Performance & Reliability**

  - Scalable to support 50000 tps
  - System uptime of up to 99.99%
  - Deployment with very low latency
  - Hosting option with AWS or GCP

- **Value Added Services**

  - Compliance Support, incl. PCI, GDPR, Card Vault etc
  - Customise the integration or payment experience
  - Control Center with elaborate analytics and reporting
  - Integration with Risk Management Solutions
  - Integration with other platforms like Subscription, E-commerce, Accounting,
    etc.

- **Enterprise Support**

  - 24x7 Email / On-call Support
  - Dedicated Relationship Manager
  - Custom dashboards with deep analytics, alerts, and reporting
  - Expert team to consult and improve business metrics

You can [try the hosted version in our sandbox][dashboard].

<!--
## Documentation

Please refer to the following documentation pages:

- Getting Started Guide [Link]
- API Reference [Link]
- Payments Fundamentals [Link]
- Installation Support [Link]
- Router Architecture [Link]
 -->

<!-- ### Sub-Crates -->

<a href="#Join-us-in-building-Hyperswitch">
  <h2 id="join-us-in-building-hyperswitch">💪 Join us in building Hyperswitch</h2>
</a>

### 🤝 Our Belief

> Payments should be open, fast, reliable and affordable to serve
> the billions of people at scale.

Globally payment diversity has been growing at a rapid pace.
There are hundreds of payment processors and new payment methods like BNPL,
RTP etc.
Businesses need to embrace this diversity to increase conversion, reduce cost
and improve control.
But integrating and maintaining multiple processors needs a lot of dev effort.
Why should devs across companies repeat the same work?
Why can't it be unified and reused? Hence, Hyperswitch was born to create that
reusable core and let companies build and customise it as per their specific requirements.

### ✨ Our Values

1. Embrace Payments Diversity: It will drive innovation in the ecosystem in
   multiple ways.
2. Make it Open Source: Increases trust; Improves the quality and reusability of
   software.
3. Be community driven: It enables participatory design and development.
4. Build it like Systems Software: This sets a high bar for Reliability,
   Security and Performance SLAs.
5. Maximise Value Creation: For developers, customers & partners.

### 🤍 Contributing

This project is being created and maintained by [Juspay](https://juspay.in),
South Asia's largest payments orchestrator/switch, processing more than 50
Million transactions per day. The solution has 1Mn+ lines of Haskell code built
over ten years.
Hyperswitch leverages our experience in building large-scale, enterprise-grade &
frictionless payment solutions.
It is built afresh for the global markets as an open-source product in Rust.
We are long-term committed to building and making it useful for the community.

The product roadmap is open for the community's feedback.
We shall evolve a prioritisation process that is open and community-driven.
We welcome contributions from the community. Please read through our
[contributing guidelines](/docs/CONTRIBUTING.md).
Included are directions for opening issues, coding standards, and notes on
development.

- We appreciate all types of contributions: code, documentation, demo creation, or some new way you want to contribute to us.
  We will reward every contribution with a Hyperswitch branded t-shirt.
- 🦀 **Important note for Rust developers**: We aim for contributions from the community across a broad range of tracks.
  Hence, we have prioritised simplicity and code readability over purely idiomatic code.
  For example, some of the code in core functions (e.g., `payments_core`) is written to be more readable than pure-idiomatic.

<a href="#Community">
  <h2 id="community">👥 Community</h2>
</a>

Get updates on Hyperswitch development and chat with the community:

- [Discord server][discord] for questions related to contributing to hyperswitch, questions about the architecture, components, etc.
- [Slack workspace][slack] for questions related to integrating hyperswitch, integrating a connector in hyperswitch, etc.
- [GitHub Discussions][github-discussions] to drop feature requests or suggest anything payments-related you need for your stack.

[discord]: https://discord.gg/wJZ7DVW8mm
[slack]: https://join.slack.com/t/hyperswitch-io/shared_invite/zt-2awm23agh-p_G5xNpziv6yAiedTkkqLg
[github-discussions]: https://github.com/juspay/hyperswitch/discussions

<div style="display: flex;  justify-content: center;">
    <div style="margin-right:10px">
    <a href="https://www.producthunt.com/posts/hyperswitch-2?utm_source=badge-top-post-badge&utm_medium=badge&utm_souce=badge-hyperswitch&#0045;2" target="_blank">
        <img src="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=375220&theme=light&period=weekly" alt="Hyperswitch - Fast, reliable, and affordable open source payments switch | Product Hunt" style="width: 250px; height: 54px;" width="250" height="54" />
    </a>
    </div>
    <div style="margin-right:10px">
    <a href="https://www.producthunt.com/posts/hyperswitch-2?utm_source=badge-top-post-topic-badge&utm_medium=badge&utm_souce=badge-hyperswitch&#0045;2" target="_blank">
        <img src="https://api.producthunt.com/widgets/embed-image/v1/top-post-topic-badge.svg?post_id=375220&theme=light&period=weekly&topic_id=267" alt="Hyperswitch - Fast, reliable, and affordable open source payments switch | Product Hunt" style="width: 250px; height: 54px;" width="250" height="54" />
    </a>
  </div>
  <div style="margin-right:10px">
    <a href="https://www.producthunt.com/posts/hyperswitch-2?utm_source=badge-top-post-topic-badge&utm_medium=badge&utm_souce=badge-hyperswitch&#0045;2" target="_blank">
        <img src="https://api.producthunt.com/widgets/embed-image/v1/top-post-topic-badge.svg?post_id=375220&theme=light&period=weekly&topic_id=93" alt="Hyperswitch - Fast, reliable, and affordable open source payments switch | Product Hunt" style="width: 250px; height: 54px;" width="250" height="54" />
    </a>
  </div>
</div>

<a href="#Bugs and feature requests">
  <h2 id="bugs-and-feature-requests">🐞 Bugs and feature requests</h2>
</a>

Please read the issue guidelines and search for [existing and closed issues].
If your problem or idea is not addressed yet, please [open a new issue].

[existing and closed issues]: https://github.com/juspay/hyperswitch/issues
[open a new issue]: https://github.com/juspay/hyperswitch/issues/new/choose

<a href="#Versioning">
  <h2 id="versioning">🔖 Versioning</h2>
</a>

Check the [CHANGELOG.md](./CHANGELOG.md) file for details.

<a href="#FAQs">
  <h2 id="FAQs">🤔 FAQs</h2>
</a>

Got more questions?
Please refer to our [FAQs page][faqs].

[faqs]: https://hyperswitch.io/docs/devSupport

<a href="#©Copyright and License">
  <h2 id="copyright-and-license">©️ Copyright and License</h2>
</a>

This product is licensed under the [Apache 2.0 License](LICENSE).

<a href="#Thanks to all contributors">
  <h2 id="Thanks to all contributors">✨ Thanks to all contributors</h2>
</a>

Thank you for your support in hyperswitch's growth. Keep up the great work! 🥂

<a href="https://github.com/juspay/hyperswitch/graphs/contributors">
  <img src="https://contributors-img.web.app/image?repo=juspay/hyperswitch" alt="Contributors"/>
</a>
