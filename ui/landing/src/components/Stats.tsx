import React from 'react';
import './Stats.css';
import { GithubIcon, ArrowRightIcon } from './Icons';

const Stats: React.FC = () => {
  const stats = [
    {
      number: '10+',
      label: 'Rust Microservices',
      description: 'From the redirect path to analytics ingestion and certificate automation'
    },
    {
      number: '100%',
      label: 'Open Source',
      description: 'MIT licensed, with the full source and architecture docs on GitHub'
    },
    {
      number: '17',
      label: 'Terraform Modules',
      description: 'A complete AWS deployment, from VPC to ECS, ready to apply'
    },
    {
      number: '0',
      label: 'Vendor Lock-In',
      description: 'Run it on your own infrastructure, with your own data, from day one'
    }
  ];

  return (
    <section className="stats section">
      <div className="container">
        <span className="section-eyebrow">By the Numbers</span>
        <h2 className="section-title">Built in the open</h2>
        <p className="section-subtitle">
          A transparent architecture you can read, run, and modify - not a black box.
        </p>

        <div className="stats-grid">
          {stats.map((stat, index) => (
            <div key={index} className="stat-card">
              <div className="stat-number">{stat.number}</div>
              <div className="stat-label">{stat.label}</div>
              <div className="stat-description">{stat.description}</div>
            </div>
          ))}
        </div>

        <div className="cta-section">
          <h3 className="cta-title">Ready to look under the hood?</h3>
          <p className="cta-subtitle">
            Read the architecture, run it locally, or deploy it to your own AWS account.
          </p>
          <div className="cta-actions">
            <a href="https://shortas.work/architecture/" target="_blank" rel="noopener noreferrer" className="btn btn-primary btn-large">
              Read the Architecture
              <ArrowRightIcon width={18} height={18} />
            </a>
            <a href="https://github.com/FlyingCow/shortas" target="_blank" rel="noopener noreferrer" className="btn btn-secondary btn-large">
              <GithubIcon width={18} height={18} />
              Star on GitHub
            </a>
          </div>
        </div>
      </div>
    </section>
  );
};

export default Stats;
