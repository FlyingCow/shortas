import React from 'react';

type IconProps = React.SVGProps<SVGSVGElement>;

const base = (children: React.ReactNode, props: IconProps) => (
  <svg
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="1.75"
    strokeLinecap="round"
    strokeLinejoin="round"
    {...props}
  >
    {children}
  </svg>
);

export const ZapIcon: React.FC<IconProps> = (props) =>
  base(<path d="M13 2 4 14h7l-1 8 9-12h-7l1-8z" />, props);

export const BarChartIcon: React.FC<IconProps> = (props) =>
  base(
    <>
      <path d="M3 21h18" />
      <rect x="5" y="14" width="3.2" height="4" rx="0.5" />
      <rect x="10.4" y="8" width="3.2" height="10" rx="0.5" />
      <rect x="15.8" y="11" width="3.2" height="7" rx="0.5" />
    </>,
    props
  );

export const LinkIcon: React.FC<IconProps> = (props) =>
  base(
    <>
      <path d="M9.5 14.5 14.5 9.5" />
      <path d="M11 6.5 12.3 5.2a3.6 3.6 0 0 1 5.1 5.1L16 11.7" />
      <path d="M13 17.5 11.7 18.8a3.6 3.6 0 0 1-5.1-5.1L8 12.3" />
    </>,
    props
  );

export const ShieldIcon: React.FC<IconProps> = (props) =>
  base(
    <>
      <path d="M12 3 4.5 5.8V11c0 5 3.3 7.8 7.5 10 4.2-2.2 7.5-5 7.5-10V5.8L12 3z" />
      <path d="M9 12.2 11.3 14.5 15.3 10" />
    </>,
    props
  );

export const GlobeIcon: React.FC<IconProps> = (props) =>
  base(
    <>
      <circle cx="12" cy="12" r="9" />
      <path d="M3 12h18" />
      <path d="M12 3c2.5 2.5 3.8 5.7 3.8 9s-1.3 6.5-3.8 9c-2.5-2.5-3.8-5.7-3.8-9s1.3-6.5 3.8-9z" />
    </>,
    props
  );

export const CodeIcon: React.FC<IconProps> = (props) =>
  base(
    <>
      <path d="m9 7-5 5 5 5" />
      <path d="m15 7 5 5-5 5" />
    </>,
    props
  );

export const GithubIcon: React.FC<IconProps> = (props) =>
  base(
    <path d="M12 2a10 10 0 0 0-3.16 19.5c.5.1.68-.22.68-.48v-1.7c-2.78.6-3.37-1.34-3.37-1.34-.46-1.15-1.11-1.46-1.11-1.46-.9-.62.07-.6.07-.6 1 .07 1.53 1.03 1.53 1.03.9 1.52 2.34 1.08 2.91.83.09-.65.35-1.08.63-1.33-2.22-.25-4.56-1.11-4.56-4.95 0-1.09.39-1.98 1.03-2.68-.1-.26-.45-1.3.1-2.7 0 0 .84-.27 2.75 1.02a9.6 9.6 0 0 1 5 0c1.91-1.3 2.75-1.02 2.75-1.02.55 1.4.2 2.44.1 2.7.64.7 1.03 1.59 1.03 2.68 0 3.85-2.35 4.7-4.58 4.94.36.31.68.92.68 1.85v2.75c0 .26.18.57.69.48A10 10 0 0 0 12 2z" />,
    props
  );

export const MenuIcon: React.FC<IconProps> = (props) =>
  base(
    <>
      <path d="M3 6h18" />
      <path d="M3 12h18" />
      <path d="M3 18h18" />
    </>,
    props
  );

export const CloseIcon: React.FC<IconProps> = (props) =>
  base(
    <>
      <path d="M18 6 6 18" />
      <path d="M6 6 18 18" />
    </>,
    props
  );

export const ArrowRightIcon: React.FC<IconProps> = (props) =>
  base(
    <>
      <path d="M5 12h14" />
      <path d="m13 6 6 6-6 6" />
    </>,
    props
  );

export const UnlockIcon: React.FC<IconProps> = (props) =>
  base(
    <>
      <rect x="4.5" y="10.5" width="13" height="10" rx="1.5" />
      <path d="M8 10.5V7a4 4 0 0 1 7.5-1.9" />
    </>,
    props
  );

export const UsersIcon: React.FC<IconProps> = (props) =>
  base(
    <>
      <circle cx="9" cy="8" r="3" />
      <path d="M3.5 20c0-3.3 2.5-6 5.5-6s5.5 2.7 5.5 6" />
      <path d="M16 8.2a3 3 0 1 1 1.3 5.6" />
      <path d="M15.3 14c2.6.3 4.7 2.8 4.7 6" />
    </>,
    props
  );

export const ServerIcon: React.FC<IconProps> = (props) =>
  base(
    <>
      <rect x="3.5" y="4" width="17" height="6.5" rx="1.5" />
      <rect x="3.5" y="13.5" width="17" height="6.5" rx="1.5" />
      <path d="M7 7.3h.01" />
      <path d="M7 16.8h.01" />
    </>,
    props
  );
