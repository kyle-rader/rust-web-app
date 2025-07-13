import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import Page from './+page.svelte';

describe('Main Page', () => {
	it('renders the main heading', () => {
		render(Page);
		
		expect(screen.getByRole('heading', { name: /rustwebapp/i })).toBeInTheDocument();
	});

	it('renders the tagline', () => {
		render(Page);
		
		expect(screen.getByText(/build - deploy - scale/i)).toBeInTheDocument();
	});

	it('renders the quick start section', () => {
		render(Page);
		
		expect(screen.getByRole('heading', { name: /quick start/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /get started/i })).toBeInTheDocument();
	});

	it('renders the documentation section', () => {
		render(Page);
		
		expect(screen.getByRole('heading', { name: /documentation/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /view docs/i })).toBeInTheDocument();
	});

	it('renders the examples section', () => {
		render(Page);
		
		expect(screen.getByRole('heading', { name: /examples/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /browse examples/i })).toBeInTheDocument();
	});

	it('includes ResponsivePad component', () => {
		const { container } = render(Page);
		
		// ResponsivePad renders a paragraph element
		const responsivePad = container.querySelector('p');
		expect(responsivePad).toBeInTheDocument();
	});

	it('has accessible button labels', () => {
		render(Page);
		
		expect(screen.getByRole('button', { name: /get started/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /view docs/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /browse examples/i })).toBeInTheDocument();
	});

	it('renders all three main options', () => {
		render(Page);
		
		// Should have three option sections
		const options = screen.getAllByRole('button');
		expect(options).toHaveLength(3);
		
		// Check for specific buttons
		expect(screen.getByRole('button', { name: /get started/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /view docs/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /browse examples/i })).toBeInTheDocument();
	});

	it('has proper section structure', () => {
		render(Page);
		
		// Check that all section headings are present
		expect(screen.getByRole('heading', { name: /quick start/i })).toBeInTheDocument();
		expect(screen.getByRole('heading', { name: /documentation/i })).toBeInTheDocument();
		expect(screen.getByRole('heading', { name: /examples/i })).toBeInTheDocument();
	});
}); 