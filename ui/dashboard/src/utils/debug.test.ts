import { debugLog, debugWarn, debugError } from './debug';

describe('debug utils', () => {
  const originalEnv = process.env.NODE_ENV;

  afterEach(() => {
    process.env.NODE_ENV = originalEnv;
    jest.restoreAllMocks();
  });

  describe('in development', () => {
    beforeEach(() => {
      process.env.NODE_ENV = 'development';
    });

    it('debugLog writes to console.log with the Shortas Debug prefix', () => {
      const spy = jest.spyOn(console, 'log').mockImplementation(() => {});
      debugLog('hello', { a: 1 });
      expect(spy).toHaveBeenCalledWith('[Shortas Debug] hello', { a: 1 });
    });

    it('debugWarn writes to console.warn with the Shortas Warning prefix', () => {
      const spy = jest.spyOn(console, 'warn').mockImplementation(() => {});
      debugWarn('careful');
      expect(spy).toHaveBeenCalledWith('[Shortas Warning] careful', '');
    });

    it('debugError writes to console.error with the Shortas Error prefix', () => {
      const spy = jest.spyOn(console, 'error').mockImplementation(() => {});
      const error = new Error('boom');
      debugError('failed', error);
      expect(spy).toHaveBeenCalledWith('[Shortas Error] failed', error);
    });
  });

  describe('outside development', () => {
    beforeEach(() => {
      process.env.NODE_ENV = 'production';
    });

    it('debugLog is silent', () => {
      const spy = jest.spyOn(console, 'log').mockImplementation(() => {});
      debugLog('hello');
      expect(spy).not.toHaveBeenCalled();
    });

    it('debugWarn is silent', () => {
      const spy = jest.spyOn(console, 'warn').mockImplementation(() => {});
      debugWarn('careful');
      expect(spy).not.toHaveBeenCalled();
    });

    it('debugError is silent', () => {
      const spy = jest.spyOn(console, 'error').mockImplementation(() => {});
      debugError('failed');
      expect(spy).not.toHaveBeenCalled();
    });
  });
});
