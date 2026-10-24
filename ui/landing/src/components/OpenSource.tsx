import React from 'react';
import './OpenSource.css';
import { UnlockIcon, UsersIcon, ServerIcon, GithubIcon } from './Icons';

const OpenSource: React.FC = () => {
  return (
    <section id="open-source" className="open-source section">
      <div className="container">
        <div className="open-source-content">
          <div className="open-source-text">
            <span className="section-eyebrow">Open Source</span>
            <h2 className="section-title">Built in the open, for developers</h2>
            <p className="section-subtitle">
              Shortas is fully open source under the MIT license, with the architecture
              and every design decision documented alongside the code.
            </p>

            <div className="open-source-features">
              <div className="os-feature">
                <div className="os-icon"><UnlockIcon width={20} height={20} /></div>
                <div className="os-content">
                  <h3>Fully Open Source</h3>
                  <p>All code is on GitHub under the MIT license - no hidden algorithms, no black boxes.</p>
                </div>
              </div>

              <div className="os-feature">
                <div className="os-icon"><UsersIcon width={20} height={20} /></div>
                <div className="os-content">
                  <h3>Community Driven</h3>
                  <p>Built by developers, for developers. Issues, discussions, and contributions are all welcome.</p>
                </div>
              </div>

              <div className="os-feature">
                <div className="os-icon"><ServerIcon width={20} height={20} /></div>
                <div className="os-content">
                  <h3>Self-Hostable</h3>
                  <p>Run it with Docker Compose locally or deploy it to AWS with the included Terraform - your data stays yours.</p>
                </div>
              </div>
            </div>

            <div className="github-actions">
              <a
                href="https://github.com/FlyingCow/shortas"
                target="_blank"
                rel="noopener noreferrer"
                className="btn btn-primary"
              >
                <GithubIcon width={18} height={18} />
                View Source Code
              </a>
              <a
                href="https://shortas.work/development/"
                target="_blank"
                rel="noopener noreferrer"
                className="btn btn-secondary"
              >
                Start Contributing
              </a>
            </div>
          </div>

          <div className="github-preview">
            <div className="github-card">
              <div className="github-header">
                <div className="github-avatar"><GithubIcon width={22} height={22} /></div>
                <div className="github-info">
                  <h4>FlyingCow/shortas</h4>
                  <p>Open-source URL shortener built with Rust</p>
                </div>
              </div>

              <div className="github-topics">
                <span className="topic">rust</span>
                <span className="topic">url-shortener</span>
                <span className="topic">self-hosted</span>
                <span className="topic">mit-license</span>
                <span className="topic">terraform</span>
                <span className="topic">clickhouse</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default OpenSource;
