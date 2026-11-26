import { ConversionDto } from '../../services/api';

/** Stands for the secret key, which is never put in a snippet. */
export const SECRET_PLACEHOLDER = '$SHORTAS_SECRET_KEY';

const DOMAIN_PLACEHOLDER = 'YOUR-SHORT-DOMAIN';

const lastStep = (c: ConversionDto) => c.steps[c.steps.length - 1]?.key ?? '';

export function scriptSnippet(c: ConversionDto, domain?: string): string {
  return `<script>(function(w){var s=w.shortas=w.shortas||{queue:[]};["track","consent"].forEach(function(m){
s[m]=s[m]||function(){s.queue.push([m].concat([].slice.call(arguments)))}})})(window)</script>
<script async src="https://${domain ?? DOMAIN_PLACEHOLDER}/.well-known/shortas/c/${c.publicKey}.js"></script>`;
}

export function stepSnippet(c: ConversionDto): string {
  const calls = c.steps.map((step) => {
    const options =
      step.key === lastStep(c)
        ? c.serverValuesOnly
          ? ', { externalId: "order-1042" }'
          : `, { value: 49.90, currency: "${c.currency}", externalId: "order-1042" }`
        : '';
    return `shortas.track("${c.publicKey}", "${step.key}"${options});`;
  });
  return `<script>\n${calls.join('\n')}\n</script>`;
}

function request(path: string, body: object, domain?: string): string {
  return `curl -X POST https://${domain ?? DOMAIN_PLACEHOLDER}/.well-known/shortas/v1/${path} \\
  -H "Authorization: Bearer ${SECRET_PLACEHOLDER}" \\
  -H "Content-Type: application/json" \\
  -d '${JSON.stringify(body)}'`;
}

export const serverSnippet = (c: ConversionDto, domain?: string): string =>
  request(
    'conversions',
    { step: lastStep(c), clickId: 'CLICK_ID', value: '49.90', currency: c.currency, externalId: 'order-1042' },
    domain
  );

export const refundSnippet = (c: ConversionDto, domain?: string): string =>
  request(
    'refunds',
    { refundId: 'refund-7', externalId: 'order-1042', value: '20', currency: c.currency },
    domain
  );
