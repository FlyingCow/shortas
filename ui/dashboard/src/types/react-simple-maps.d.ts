declare module 'react-simple-maps' {
  import { ReactNode } from 'react';

  export interface ComposableMapProps {
    projection?: string;
    projectionConfig?: {
      scale?: number;
      center?: [number, number];
    };
    width?: number;
    height?: number;
    style?: React.CSSProperties;
    children?: ReactNode;
  }

  export interface GeographiesProps {
    geography: string | Record<string, any>;
    children: (args: { geographies: any[] }) => ReactNode;
  }

  export interface GeographyProps {
    geography: any;
    fill?: string;
    stroke?: string;
    strokeWidth?: number;
    onMouseEnter?: (event: any) => void;
    onMouseLeave?: () => void;
    style?: {
      default?: React.CSSProperties;
      hover?: React.CSSProperties;
      pressed?: React.CSSProperties;
    };
  }

  export interface ZoomableGroupProps {
    children?: ReactNode;
  }

  export const ComposableMap: React.FC<ComposableMapProps>;
  export const Geographies: React.FC<GeographiesProps>;
  export const Geography: React.FC<GeographyProps>;
  export const ZoomableGroup: React.FC<ZoomableGroupProps>;
}
