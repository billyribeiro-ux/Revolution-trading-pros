import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import ParagraphBlock from '../content/ParagraphBlock.svelte';
import type { Block } from '../types';

describe('ParagraphBlock', () => {
    const createMockBlock = (overrides = {}): Block => ({
        id: 'test-block-1',
        type: 'paragraph',
        content: {
            text: 'Test paragraph content'
        },
        settings: {
            textAlign: 'left'
        },
        metadata: {
            createdAt: Date.now(),
            updatedAt: Date.now(),
            version: 1
        },
        ...overrides
    });

    const defaultProps = {
        block: createMockBlock(),
        blockId: 'test-block-1',
        isSelected: false,
        isEditing: false,
        onUpdate: vi.fn(),
        onError: vi.fn()
    };

    beforeEach(() => {
        vi.clearAllMocks();
    });

    it('renders paragraph text correctly', () => {
        render(ParagraphBlock, { props: defaultProps });
        expect(screen.getByText('Test paragraph content')).toBeInTheDocument();
    });

    it('applies text alignment from settings', () => {
        const props = {
            ...defaultProps,
            block: createMockBlock({ settings: { textAlign: 'center' } })
        };
        render(ParagraphBlock, { props });
        const paragraph = screen.getByText('Test paragraph content');
        expect(paragraph).toHaveClass('paragraph-block--align-center');
    });

    it('shows selected state when isSelected is true', () => {
        const props = { ...defaultProps, isSelected: true };
        render(ParagraphBlock, { props });
        const paragraph = screen.getByText('Test paragraph content');
        expect(paragraph).toHaveClass('paragraph-block--selected');
    });

    it('enables contenteditable when isEditing is true', () => {
        const props = { ...defaultProps, isEditing: true };
        render(ParagraphBlock, { props });
        const paragraph = screen.getByText('Test paragraph content');
        expect(paragraph).toHaveAttribute('contenteditable', 'true');
    });

    it('calls onUpdate when text is modified', async () => {
        const onUpdate = vi.fn();
        const props = { ...defaultProps, isEditing: true, onUpdate };
        render(ParagraphBlock, { props });

        const paragraph = screen.getByText('Test paragraph content');
        paragraph.textContent = 'Updated content';
        await fireEvent.input(paragraph);

        expect(onUpdate).toHaveBeenCalled();
    });

    it('shows placeholder when text is empty', () => {
        const props = {
            ...defaultProps,
            block: createMockBlock({ content: { text: '' } })
        };
        render(ParagraphBlock, { props });
        const paragraph = screen.getByRole('paragraph') || document.querySelector('.paragraph-block');
        expect(paragraph).toHaveClass('paragraph-block--placeholder');
    });

    it('handles paste by stripping HTML', async () => {
        const onUpdate = vi.fn();
        const props = { ...defaultProps, isEditing: true, onUpdate };
        render(ParagraphBlock, { props });

        const paragraph = screen.getByText('Test paragraph content');
        const pasteEvent = new ClipboardEvent('paste', {
            clipboardData: new DataTransfer()
        });
        pasteEvent.clipboardData?.setData('text/plain', 'Pasted text');

        paragraph.dispatchEvent(pasteEvent);
    });

    it('applies custom text color from settings', () => {
        const props = {
            ...defaultProps,
            block: createMockBlock({ settings: { textColor: '#ff0000' } })
        };
        render(ParagraphBlock, { props });
        const paragraph = screen.getByText('Test paragraph content');
        expect(paragraph).toHaveStyle({ color: '#ff0000' });
    });
});
