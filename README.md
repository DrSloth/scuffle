<div align="center">
    <h1>Scuffle</h1>
    <h3>A next-generation open-source cloud provider</h3>
</div>

<p align="center">
    <a href="https://twitter.com/scufflecloud"><img height="30" src="https://img.shields.io/badge/Twitter-000000?style=for-the-badge&logo=x&logoColor=white"></a>
    &nbsp;
    <a href="https://bsky.app/profile/scuffle.cloud"><img height="30" src="https://img.shields.io/badge/Bluesky-00A0FF?style=for-the-badge&logo=bluesky&logoColor=white"></a>
    &nbsp;
    <a href="https://discord.gg/scuffle"><img height="30" src="https://img.shields.io/badge/Discord-5865f2?style=for-the-badge&logo=discord&logoColor=white"></a>
    &nbsp;
    <a href="https://linkedin.com/company/scufflecloud"><img height="30" src="https://img.shields.io/badge/LinkedIn-0A66C2?style=for-the-badge&logo=linkedin&logoColor=white"></a>
</p>

<p align="center">
    <a href="https://opencollective.com/scuffle"><img height="30" src="https://img.shields.io/badge/support us-764bd1?style=for-the-badge&logo=opencollective&logoColor=white&labelColor=gray"/></a>
    &nbsp;
    <a href="https://jira.scuffle.cloud"><img height="30" src="https://img.shields.io/badge/issue tracker-0052CC?style=for-the-badge&logo=jira&logoColor=white&labelColor=gray"/></a>
    &nbsp;
    <a href="#"><img height="30" src="https://img.shields.io/badge/made with ❤️-f0a63e?style=for-the-badge&logo=rust&logoColor=white&labelColor=gray"/></a>
</p>

<p align="center">
    <a href="https://codecov.io/gh/ScuffleCloud/scuffle"><img height="30" src="https://img.shields.io/codecov/c/github/ScuffleCloud/scuffle?logo=codecov&token=LJCYSZR4IV&style=for-the-badge"/></a>
    &nbsp;
    <a href="https://github.com/ScuffleCloud/scuffle/actions?query=branch%3Amain"><img height="30" src="https://img.shields.io/github/check-runs/ScuffleCloud/Scuffle/main?style=for-the-badge&logo=githubactions&logoColor=white"/></a>
</p>

> [!WARNING]  
> This repository is under active development and may not be stable.

---

Welcome to **Scuffle**—a next-generation **open-source cloud provider**! 🚀

We're on a mission to revolutionize **video streaming solutions** with cutting-edge tools and libraries. Dive in and explore what we have to offer! 🔗 [Visit our website](https://scuffle.cloud) to learn more.

## 🛠️ Crates

This repository houses a collection of crates, purpose-built libraries designed to simplify development and enhance functionality:

- ⚡ **[scuffle-batching](./crates/batching)**:  Optimized batching and dataloading for external services.
- 🚀 **[scuffle-bootstrap](./crates/bootstrap)**:  A utility crate for creating binaries.
  - 🔧 **[scuffle-bootstrap-derive](./crates/bootstrap/derive)**:  Derive macros for `scuffle-bootstrap`.
  - 🔭 **[scuffle-bootstrap-telemetry](./crates/bootstrap/telemetry)**:  Telemetry utilities for `scuffle-bootstrap`.
- 📦 **[scuffle-bytes-util](./crates/bytes-util)**:  Some helpful utilities for working with bits and bytes.
- 🧭 **[scuffle-context](./crates/context)**:  Go-like context utilities for Rust.
- ⌛ **[scuffle-future-ext](./crates/future-ext)**:  Extensions for working with futures.
- 🦈 **[scuffle-http](./crates/http)**:  A high-performance HTTP server supporting HTTP/1.1, HTTP/2, and HTTP/3.
- 📊 **[scuffle-metrics](./crates/metrics)**:  Helper crate to instrument your code with metrics.
  - 🔧 **[scuffle-metrics-derive](./crates/metrics/derive)**:  Derive macros for `scuffle-metrics`.
- 📦 **[postcompile](./crates/postcompile)**:  A macro for compiling Rust code at runtime. Useful for snapshot testing.
- 📈 **[scuffle-pprof](./crates/pprof)**:  Helper crate for adding pprof support to your application.
- ⚙️ **[scuffle-settings](./crates/settings)**:  Tools for managing configuration from environment variables or config files.
- 📶 **[scuffle-signal](./crates/signal)**:  Ergonomic async signal handling.

### 🎥 Multimedia Crates

Apart from utility crates, we also offer a range of multimedia crates to encode, decode, and transmux media files and streams:

- 🔉 **[scuffle-aac](./crates/aac)**:  A crate for decoding AAC audio headers.
- 🗃️ **[scuffle-amf0](./crates/amf0)**:  A pure-rust implementation of AMF0 encoder and decoder.
- 🎥 **[scuffle-av1](./crates/av1)**:  A crate for decoding and encoding AV1 video headers.
- 🧮 **[scuffle-expgolomb](./crates/expgolomb)**:  A set of helper functions to encode and decode exponential-golomb values.
- 🎞️ **[scuffle-ffmpeg](./crates/ffmpeg)**:  A safe Rust wrapper around FFmpeg C-bindings.
- 🎥 **[scuffle-flv](./crates/flv)**:  A pure Rust implementation of the FLV format, allowing for demuxing of FLV files or streams.
<!-- - 🎥 **[scuffle-h264](./crates/h264)**: -->
<!-- - 🎥 **[scuffle-h265](./crates/h265)**: -->
<!-- - 🎥 **[scuffle-mp4](./crates/mp4)**: -->
<!-- - 🎥 **[scuffle-rtmp](./crates/rtmp)**: -->
<!-- - 🎥 **[scuffle-transmuxer](./crates/transmuxer)**: -->

---

## 🤝 Join the Scuffle Community

We ❤️ contributions! Check out our [**CONTRIBUTING.md**](./CONTRIBUTING.md) for detailed guidelines on submitting improvements or bug fixes.

Join our [Discord](https://discord.gg/scuffle) to chat with the team and other contributors.

### Sign the Contributor License Agreement (CLA)

To contribute, please sign our [Contributor License Agreement](./CLA.md).  
It's quick and easy—[sign here](https://cla.scuffle.cloud) before submitting a pull request.

### Code of Conduct

We believe in fostering an inclusive and respectful community.  
Please read our [**Code of Conduct**](./CODE_OF_CONDUCT.md) for more details.

## 💖 Support Us

If you find Scuffle valuable, consider supporting us on [**Open Collective**](https://opencollective.com/scuffle). Your contributions help us continue to improve and maintain this open-source project.

[![Open Collective](https://a11ybadges.com/badge?logo=opencollective)](https://opencollective.com/scuffle)

---

## 📜 Licensing Overview

Scuffle uses **different licenses** for various components. Here's a quick general overview:

- **Libraries**: Licensed under [MIT](./LICENSE.MIT) or [Apache-2.0](./LICENSE.Apache-2.0).
- **Executable Binaries**: Licensed under [AGPL-3.0](./LICENSE.AGPL-3.0).

Each component includes specific licensing details in its `README.md` and corresponding license files. For any questions, feel free to [open an issue](https://github.com/ScuffleCloud/scuffle/issues) or email us at [legal@scuffle.cloud](mailto:legal@scuffle.cloud).

---

## 🛡️ Security Matters

Your security is our priority. 🔒 Refer to our [**Security Policy**](./.github/SECURITY.md) for guidelines on reporting vulnerabilities.

---

## 📋 Additional Resources

- 🔍 [**Public Issue Tracker**](https://jira.scuffle.cloud)
- 📊 [**Kanban Board**](https://scuffle.notion.site)
- 📰 [**Scuffle Blog**](https://bytes.scuffle.cloud)

---

## ⭐ Star History

[![Star History Chart](https://api.star-history.com/svg?repos=scufflecloud/scuffle&type=Date)](https://star-history.com/#scufflecloud/scuffle&Date)

---

*Thank you for being part of our journey! Let's build something amazing together.* ✨
