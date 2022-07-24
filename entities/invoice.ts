type Contact = string;
type Note = string;
type Id = string;

export interface Invoice {
	id: Id;
	note: Note;
	to: Contact;
	from: Contact;
	items: InvoiceItem[];
}

export interface InvoiceItem {
	description: string;
	quantity: number;
	rate: number;
}

export function makeInvoice(invoice: Partial<Invoice>): Invoice {
	return {
		id: '',
		note: '',
		from: '',
		to: '',
		items: [],
		...invoice,
	};
}

export function makeInvoiceItem(item: Partial<InvoiceItem>): InvoiceItem {
	return {
		description: '',
		quantity: 1,
		rate: 0,
		...item,
	};
}

export const INVOICE_URL = 'https://invoice-generator.com/#/invoices';

interface InvoiceSelectorMap extends Record<keyof Invoice, string> {
	addItem: string;
}

export const INVOICE_SELECTOR_MAP: InvoiceSelectorMap = {
	id: '.subtitle input',
	to: '.contact.to textarea',
	from: '.contact.from textarea',
	note: '.notes-holder textarea',
	items: '.items-holder .items-table>.item-row',
	addItem: '.new-line button',
};
