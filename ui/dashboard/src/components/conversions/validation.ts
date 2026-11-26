import { ConversionInputDto } from '../../services/api';

export const MAX_STEPS = 20;
const MAX_ORIGINS = 20;

const validName = (name: string) => name.trim().length > 0 && name.length <= 100;

const validOrigin = (origin: string) => {
  try {
    const url = new URL(origin);
    return ['http:', 'https:'].includes(url.protocol) && url.origin === origin;
  } catch {
    return false;
  }
};

/** Errors by field, named as the API names them. The API checks again. */
export function validate(input: ConversionInputDto): Record<string, string> {
  const errors: Record<string, string> = {};
  const check = (ok: boolean, field: string, message: string) => {
    if (!ok) errors[field] = message;
  };

  check(validName(input.name), 'name', 'Enter a name of up to 100 characters');
  check(/^[A-Z]{3}$/.test(input.currency), 'currency', 'Enter a currency code, such as EUR');
  check(
    Number.isInteger(input.windowDays) && input.windowDays >= 1 && input.windowDays <= 90,
    'windowDays',
    'Enter 1 to 90 days'
  );
  check(
    input.allowedOrigins.length <= MAX_ORIGINS && input.allowedOrigins.every(validOrigin),
    'allowedOrigins',
    `Enter up to ${MAX_ORIGINS} origins such as https://shop.example, without a path`
  );
  check(!input.routeIds || input.routeIds.length > 0, 'routeIds', 'Choose at least one route');
  check(
    input.steps.length >= 1 && input.steps.length <= MAX_STEPS,
    'steps',
    `Add 1 to ${MAX_STEPS} steps`
  );

  const keys = new Set<string>();
  input.steps.forEach((step, i) => {
    if (!/^[a-z0-9_-]{1,64}$/.test(step.key)) {
      errors[`steps[${i}].key`] = 'Use a-z, 0-9, - and _';
    } else if (keys.has(step.key)) {
      errors[`steps[${i}].key`] = 'Another step has this key';
    }
    keys.add(step.key);
    check(validName(step.name), `steps[${i}].name`, 'Enter a name');
    check(
      !step.defaultValue || /^\d{1,14}(\.\d{1,4})?$/.test(step.defaultValue),
      `steps[${i}].defaultValue`,
      'Enter a number with up to 4 decimals'
    );
  });
  return errors;
}

/** The fields an API error names. */
export function apiErrors(err: any): Record<string, string> {
  const { status, data } = err?.response ?? {};
  if (status !== 400 || !data?.details) return {};
  return Object.fromEntries(
    String(data.details)
      .split(',')
      .map((field) => [
        field,
        field === 'domainId'
          ? 'This domain must be shared or belong to the workspace owner'
          : field === 'routeIds'
            ? 'Some routes are no longer in this workspace'
            : 'Invalid value',
      ])
  );
}

export const errorMessage = (err: any, fallback: string): string =>
  err?.response?.data?.message || fallback;
