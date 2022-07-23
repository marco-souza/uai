import { Command } from 'cliffy';
import { handler } from '@services/invoice.ts';

export function addInvoiceCmd(cli: Command) {
	cli
		.description('Generate an invoice')
		.command('invoice, inv', 'create invoice')
		// .option('-c, --clean', 'Return unformatted number')
		// .option('-t, --to [currency:string]', 'Specify a output currency', {
		// 	default: 'BRL',
		// })
		// .arguments('<:number>')
		.action(handler);
}
