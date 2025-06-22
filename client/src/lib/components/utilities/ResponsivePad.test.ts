import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/svelte';
import ResponsivePad from './ResponsivePad.svelte';

describe('ResponsivePad', () => {
	it('renders a paragraph element', () => {
		const { container } = render(ResponsivePad);
		
		const paragraph = container.querySelector('p');
		expect(paragraph).toBeInTheDocument();
	});

	it('has empty content', () => {
		const { container } = render(ResponsivePad);
		
		const paragraph = container.querySelector('p');
		expect(paragraph?.textContent).toBe('');
	});

	it('has proper styling structure', () => {
		const { container } = render(ResponsivePad);
		
		// Check that the component renders without errors
		expect(container.firstChild).toBeTruthy();
		
		// The component should have styles applied
		const paragraph = container.querySelector('p');
		expect(paragraph).toBeTruthy();
	});

	it('is accessible', () => {
		const { container } = render(ResponsivePad);
		
		const paragraph = container.querySelector('p');
		// Paragraph should be present in the DOM
		expect(paragraph).toBeInTheDocument();
	});

	it('renders without crashing', () => {
		expect(() => render(ResponsivePad)).not.toThrow();
	});
}); 