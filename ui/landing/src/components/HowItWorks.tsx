import React from 'react';
import './HowItWorks.css';

const HowItWorks: React.FC = () => {
  const steps = [
    {
      step: '01',
      title: 'Paste Your URL',
      description: 'Simply paste your long URL into our shortening tool. No registration required for basic use.'
    },
    {
      step: '02',
      title: 'Customize & Generate',
      description: 'Optionally customize your short link with a branded domain or custom alias, then generate.'
    },
    {
      step: '03',
      title: 'Share & Track',
      description: 'Share your short link anywhere and track its performance with detailed analytics.'
    }
  ];

  return (
    <section id="how-it-works" className="how-it-works section">
      <div className="container">
        <h2 className="section-title">How It Works</h2>
        <p className="section-subtitle">
          Get started in seconds with our simple three-step process
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
          <h3 className="demo-title">Try it now!</h3>
          <div className="url-shortener-demo">
            <input 
              type="url" 
              placeholder="Paste your long URL here..." 
              className="url-input-demo"
            />
            <button className="btn btn-primary">Shorten URL</button>
          </div>
        </div>
      </div>
    </section>
  );
};

export default HowItWorks;
