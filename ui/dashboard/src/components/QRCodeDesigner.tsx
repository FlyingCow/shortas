import React, { useRef, useEffect, useState, useCallback } from 'react';
import QRCodeStyling, { type DotType, type Options } from 'qr-code-styling';
import { QrCode, Download, ImagePlus, Palette } from 'lucide-react';
import './QRCodeDesigner.css';

const DOT_STYLES: { value: DotType; label: string }[] = [
  { value: 'square', label: 'Square' },
  { value: 'rounded', label: 'Rounded' },
  { value: 'extra-rounded', label: 'Extra rounded' },
  { value: 'dots', label: 'Dots' },
  { value: 'classy', label: 'Classy' },
  { value: 'classy-rounded', label: 'Classy rounded' },
];

const PRESET_COLORS = [
  '#000000',
  '#1e3a5f',
  '#3b82f6',
  '#059669',
  '#7c3aed',
  '#dc2626',
  '#ea580c',
  '#ca8a04',
];

const DEFAULT_DATA = 'https://example.com';

interface QRCodeDesignerProps {
  /** When set, QR content is this URL and the content input is read-only */
  url?: string;
}

const QRCodeDesigner: React.FC<QRCodeDesignerProps> = ({ url }) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const qrRef = useRef<QRCodeStyling | null>(null);
  const [data, setData] = useState(DEFAULT_DATA);
  const effectiveData = url !== undefined && url !== '' ? url : data;
  const [dotStyle, setDotStyle] = useState<DotType>('rounded');
  const [fgColor, setFgColor] = useState('#000000');
  const [bgColor, setBgColor] = useState('#ffffff');
  const [size, setSize] = useState(280);
  const [centerImage, setCenterImage] = useState<string | undefined>(undefined);
  const [imageFile, setImageFile] = useState<File | null>(null);

  const buildOptions = useCallback(
    (): Partial<Options> => ({
      width: size,
      height: size,
      type: 'svg',
      data: effectiveData || ' ',
      dotsOptions: { color: fgColor, type: dotStyle },
      backgroundOptions: { color: bgColor },
      cornersSquareOptions: { type: dotStyle === 'dots' ? 'dot' : 'square', color: fgColor },
      cornersDotOptions: { type: dotStyle === 'dots' ? 'dot' : 'square', color: fgColor },
      qrOptions: { errorCorrectionLevel: centerImage ? 'H' : 'M' },
      image: centerImage,
      imageOptions: centerImage
        ? { hideBackgroundDots: true, imageSize: 0.4, margin: 4 }
        : { hideBackgroundDots: false, imageSize: 0, margin: 0 },
    }),
    [effectiveData, dotStyle, fgColor, bgColor, size, centerImage]
  );

  useEffect(() => {
    if (!containerRef.current) return;
    const opts = buildOptions();
    if (!qrRef.current) {
      qrRef.current = new QRCodeStyling(opts);
      qrRef.current.append(containerRef.current);
    } else {
      qrRef.current.update(opts);
    }
  }, [buildOptions]);

  useEffect(() => {
    return () => {
      if (centerImage) URL.revokeObjectURL(centerImage);
    };
  }, [centerImage]);

  const handleImageChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    if (!file.type.startsWith('image/')) {
      alert('Please select an image file (PNG, JPG, SVG, etc.).');
      return;
    }
    if (centerImage) URL.revokeObjectURL(centerImage);
    setImageFile(file);
    setCenterImage(URL.createObjectURL(file));
    e.target.value = '';
  };

  const clearImage = () => {
    if (centerImage) URL.revokeObjectURL(centerImage);
    setCenterImage(undefined);
    setImageFile(null);
  };

  const handleDownload = async () => {
    if (!qrRef.current) return;
    await qrRef.current.download({ name: 'qrcode', extension: 'svg' });
  };

  return (
    <div className="qr-designer">
      <div className="qr-designer-header">
        <h3 className="qr-designer-title">
          <QrCode size={18} />
          QR Code Designer
        </h3>
        <p className="qr-designer-desc">Create a custom QR code and download as SVG</p>
      </div>

      <div className="qr-designer-body">
        <div className="qr-designer-controls">
          <div className="qr-control-group">
            <label className="qr-control-label">Content (URL or text)</label>
            <input
              type="text"
              className="qr-input"
              value={effectiveData}
              onChange={(e) => setData(e.target.value)}
              placeholder="https://..."
              readOnly={url !== undefined && url !== ''}
              style={url ? { opacity: 0.9, cursor: 'default' } : undefined}
            />
          </div>

          <div className="qr-control-group">
            <label className="qr-control-label">
              <Palette size={14} />
              Dots style
            </label>
            <div className="qr-dot-styles">
              {DOT_STYLES.map(({ value, label }) => (
                <button
                  key={value}
                  type="button"
                  className={`qr-dot-btn ${dotStyle === value ? 'active' : ''}`}
                  onClick={() => setDotStyle(value)}
                >
                  {label}
                </button>
              ))}
            </div>
          </div>

          <div className="qr-control-group qr-color-row">
            <div className="qr-color-field">
              <label className="qr-control-label">Foreground</label>
              <div className="qr-color-inputs">
                <input
                  type="color"
                  className="qr-color-swatch"
                  value={fgColor}
                  onChange={(e) => setFgColor(e.target.value)}
                />
                <input
                  type="text"
                  className="qr-color-text"
                  value={fgColor}
                  onChange={(e) => setFgColor(e.target.value)}
                />
              </div>
            </div>
            <div className="qr-color-field">
              <label className="qr-control-label">Background</label>
              <div className="qr-color-inputs">
                <input
                  type="color"
                  className="qr-color-swatch"
                  value={bgColor}
                  onChange={(e) => setBgColor(e.target.value)}
                />
                <input
                  type="text"
                  className="qr-color-text"
                  value={bgColor}
                  onChange={(e) => setBgColor(e.target.value)}
                />
              </div>
            </div>
          </div>

          <div className="qr-control-group qr-preset-colors">
            <label className="qr-control-label">Preset colors</label>
            <div className="qr-preset-row">
              {PRESET_COLORS.map((color) => (
                <button
                  key={color}
                  type="button"
                  className="qr-preset-swatch"
                  style={{ backgroundColor: color }}
                  onClick={() => setFgColor(color)}
                  title={color}
                />
              ))}
            </div>
          </div>

          <div className="qr-control-group">
            <label className="qr-control-label">Size (px)</label>
            <input
              type="range"
              className="qr-size-slider"
              min={120}
              max={512}
              step={8}
              value={size}
              onChange={(e) => setSize(Number(e.target.value))}
            />
            <span className="qr-size-value">{size}px</span>
          </div>

          <div className="qr-control-group">
            <label className="qr-control-label">
              <ImagePlus size={14} />
              Center image (logo)
            </label>
            <div className="qr-image-actions">
              <label className="qr-btn qr-btn-secondary">
                <input
                  type="file"
                  accept="image/*"
                  className="qr-file-input"
                  onChange={handleImageChange}
                />
                Choose image
              </label>
              {centerImage && (
                <button type="button" className="qr-btn qr-btn-ghost" onClick={clearImage}>
                  Remove
                </button>
              )}
            </div>
            {imageFile && (
              <p className="qr-image-name">{imageFile.name}</p>
            )}
          </div>

          <div className="qr-control-group qr-download-wrap">
            <button
              type="button"
              className="qr-btn qr-btn-primary"
              onClick={handleDownload}
            >
              <Download size={16} />
              Download SVG
            </button>
          </div>
        </div>

        <div className="qr-designer-preview">
          <div className="qr-preview-inner" ref={containerRef} />
        </div>
      </div>
    </div>
  );
};

export default QRCodeDesigner;
