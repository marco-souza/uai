import { Input } from 'https://x.nest.land/cliffy@0.20.1/mod.ts';
import puppeteer, { Browser, Page } from 'puppeteer';
import {
	Invoice,
	INVOICE_SELECTOR_MAP,
	INVOICE_URL,
	InvoiceItem,
	makeInvoice,
	makeInvoiceItem,
} from '../entities/invoice.ts';

export interface InvoiceOptions {}

// use-case: setup invoice config
export async function handler() {
	console.log('Creating a new invoice...\n');

	try {
		const browser = await puppeteer.launch();
		const page = await browser.newPage();

		const invoice = await makeInvoiceService(browser, page);

		await invoice.setupInvoice();
		await invoice.closeBrowser();
	} catch (error) {
		console.error(error);
	}
}

async function makeInvoiceService(browser: Browser, page: Page) {
	return {
		setupInvoice: () => setupInvoice(page),
		closeBrowser: () => closeBrowser(browser),
	};
}

async function setupInvoice(page: Page) {
	const invoice = await promptInvoiceInfo();

  console.log(invoice)

	await page.goto(INVOICE_URL);
	await page.waitForSelector(INVOICE_SELECTOR_MAP.id);
	await page.click('.got-it');

	// TODO: add invoice line items

	// add invoice items configs to page
	mapInvoiceToSelectors(invoice, page);

  console.log("printing invoice")
	await page.screenshot({ path: 'example.png' });
	// TODO: save invoice config to config file
}

const MIN_LEN = 3;
async function promptInvoiceInfo() {
	const id: Invoice['id'] = await Input.prompt({
		message: 'Invoice ID',
		default: '1', // TODO: load from last config
	});

	const to: Invoice['to'] = await Input.prompt({
		message: 'To',
		minLength: MIN_LEN,
	});

	const from: Invoice['from'] = await Input.prompt({
		message: 'From',
		minLength: MIN_LEN,
	});

	const note: Invoice['note'] = await Input.prompt({
		message: 'Note',
		default: '',
	});

  console.log("\nPlease type the items of the invoice");
  const items = await promptInvoiceItems();

	return makeInvoice({ id, to, from, note, items });
}

async function mapInvoiceToSelectors(invoice: Invoice, page: Page) {
	console.log('mapping invoice to selectors');

	await Promise.all([
		page.$eval(
			INVOICE_SELECTOR_MAP.id,
			(el, id) => (el.value = id),
			invoice.id,
		),
		page.$eval(
			INVOICE_SELECTOR_MAP.to,
			(el, to) => (el.value = to),
			invoice.to,
		),
		page.$eval(
			INVOICE_SELECTOR_MAP.from,
			(el, from) => (el.value = from),
			invoice.from,
		),
		page.$eval(
			INVOICE_SELECTOR_MAP.note,
			(el, note) => (el.value = note),
			invoice.note,
		),
	]);

	await Promise.all(invoice.items.map(async (item, index) => {
    // TODO: improve index selector
    const selectorValues = [
      [
				`${INVOICE_SELECTOR_MAP.items}:nth-child(${index + 1}) .quantity input`,
				item.quantity.toString(),
      ],
      [
				`${INVOICE_SELECTOR_MAP.items}:nth-child(${index + 1}) .unit_cost input`,
				item.rate.toString(),
      ],
      [
				`${INVOICE_SELECTOR_MAP.items}:nth-child(${index + 1}) .name textarea`,
				item.description,
      ],
    ]

    const promiseList = []
    for (const [selector, value] of selectorValues) {
      promiseList.push(
        page.$eval(selector, (el, value) => (el.value = value), value)
      )
    }

    return await Promise.all(promiseList)
  }));
}

async function promptInvoiceItems(
	items: InvoiceItem[] = [],
): Promise<InvoiceItem[]> {
	const item = await addInvoiceItem();

	const addAnother = await Input.prompt({
		message: 'Add another item?',
		default: 'y',
	});

  const newList = [...items, item]
	if (addAnother.toLowerCase() === 'y') {
		return promptInvoiceItems(newList);
	}

	return newList;
}

async function addInvoiceItem() {
  const now = new Date()
	const description: InvoiceItem['description'] = await Input.prompt({
		message: 'Description',
		minLength: MIN_LEN,
    default: `Services provided during ${now.getMonth()}/${now.getFullYear()}`,
	});

	const quantity = await Input.prompt({
		message: 'Quantity',
		default: '1',
	});

	const rate = await Input.prompt({
		message: 'Rate',
		default: '0',
	});

	return makeInvoiceItem({
		description,
		quantity: Number(quantity),
		rate: Number(rate),
	});
}

async function downloadInvoice() {}

async function closeBrowser(browser: Browser) {
	browser.close();
}
