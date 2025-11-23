<a id="readme-top"></a>

<!-- Project Shields -->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/garnetraven/rustyboi">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">RustyBoi</h3>

  <p align="center">
    A general purpose rust based discord bot.
    <br />
    <a href="https://github.com/github_username/repo_name"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/github_username/repo_name/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    &middot;
    <a href="https://github.com/github_username/repo_name/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

RustyBoi is a feature-rich Discord bot designed to enhance server communities with user progression, economy systems, and detailed analytics. Built with Rust for maximum performance and reliability, it scales effortlessly for communities of 100-200+ users.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

Get RustyBoi up and running in your server with these simple steps.

If you have a large community and or do not want to self host your bot. We offer a paid solution to host and scale for your community. Visit rustyboi.com

### Prerequisites

Before you begin, ensure you have the following installed:

#### For Docker Setup (Recommended)
* Docker Desktop or Docker Engine
  ```sh
  # Linux
  curl -fsSL https://get.docker.com -o get-docker.sh
  sh get-docker.sh
  
  # macOS/Windows - Download Docker Desktop from docker.com
  ```
* Docker Compose (usually included with Docker Desktop)

#### For Local Development
* Rust 1.70 or higher
  ```sh
  # Linux/macOS
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
  
  # Windows - Download from https://rustup.rs/
  ```

* PostgreSQL 12 or higher
  ```sh
  # Ubuntu/Debian
  sudo apt update
  sudo apt install postgresql
  
  # macOS (Homebrew)
  brew install postgresql@15
  brew services start postgresql@15
  
  # Windows - Download from postgresql.org
  ```

* Discord Bot Token - [Create a bot application](https://discord.com/developers/applications)

### Installation

1. **Clone the repository**
   ```sh
   git clone https://github.com/garnetraven/rustyboi.git
   cd rustyboi
   ```

2. **Create environment file**
   ```sh
   cp .env.example .env
   ```

3. **Configure your `.env` file**
   ```env
   # Discord Configuration
   DISCORD_TOKEN=your_discord_bot_token_here
   
   # Database Configuration
   DATABASE_URL=postgresql://discord_bot:secure_password@localhost/discord_bot
   ```

4. **Choose your setup method:**
   - [Docker Setup](#docker-setup) (Recommended for quick start)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Corbin Lienau - GitHub: GarnetRaven - 13lienau@gmail.com

Project Link: [https://github.com/garnetraven/rustyboi](https://github.com/garnetraven/rustyboi)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

[contributors-shield]: https://img.shields.io/github/contributors/garnetraven/rustyboi.svg?style=for-the-badge
[contributors-url]: https://github.com/garnetraven/rustyboi/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/garnetraven/rustyboi.svg?style=for-the-badge
[forks-url]: https://github.com/garnetraven/rustyboi/network/members
[stars-shield]: https://img.shields.io/github/stars/garnetraven/rustyboi.svg?style=for-the-badge
[stars-url]: https://github.com/garnetraven/rustyboi/stargazers
[issues-shield]: https://img.shields.io/github/issues/garnetraven/rustyboi.svg?style=for-the-badge
[issues-url]: https://github.com/garnetraven/rustyboi/issues
[license-shield]: https://img.shields.io/github/license/garnetraven/rustyboi.svg?style=for-the-badge
[license-url]: https://github.com/garnetraven/rustyboi/blob/main/LICENSE.txt
