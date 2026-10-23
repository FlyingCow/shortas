import React from 'react';
import './HowItWorks.css';
import { ArrowRightIcon } from './Icons';

const HowItWorks: React.FC = () => {
  const steps = [
    {
      step: '01',
      title: 'Deploy It',
      description: 'Spin up the stack with Docker Compose locally, or deploy it to AWS with the included Terraform modules.'
    },
    {
      step: '02',
      title: 'Add a Route',
      description: 'Create a short link through the dashboard or the REST API - with an optional custom domain and routing rules.'
    },
    {
      step: '03',
      title: 'Share & Track',
      description: 'Share the link anywhere. Every click streams into ClickHouse so you can see traffic as it happens.'
    }
  ];

  return (
    <section id="how-it-works" className="how-it-works section">
      <div className="container">
        <span className="section-eyebrow">How It Works</span>
        <h2 className="section-title">From clone to first click</h2>
        <p className="section-subtitle">
          A self-hosted stack you control end to end - no account, no vendor lock-in.
        </p>

        <div className="steps-container">
          {steps.map((step, index) => (
            <div key={index} className="step">
              <div className="step-number">{step.step}</div>
              <div className="step-content">
                <h3 className="step-title">{step.title}</h3>
                <p className="step-description">{step.description}</p>
              </div>
              {index < steps.length - 1 && <div className="step-connector"></div>}
            </div>
          ))}
        </div>

        <div className="demo-section">
          <h3 className="demo-title">Ready to try it?</h3>
          <p className="demo-subtitle">The getting-started guide walks through the full local setup in a few minutes.</p>
          <a href="https://shortas.work/getting-started/" target="_blank" rel="noopener noreferrer" className="btn btn-primary btn-large">
            Read the Getting Started Guide
            <ArrowRightIcon width={18} height={18} />
          </a>
        </div>
      </div>
    </section>
  );
};

export default HowItWorks;
