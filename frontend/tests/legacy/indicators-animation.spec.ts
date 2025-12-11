import { test, expect } from '@playwright/test';

test('IndicatorsSection chart animation', async ({ page }) => {
  const logs: string[] = [];
  const errors: string[] = [];
  
  page.on('console', msg => {
    const text = msg.text();
    logs.push(`[${msg.type()}] ${text}`);
    if (msg.type() === 'error') errors.push(text);
  });
  
  await page.goto('http://localhost:5174/');
  await page.waitForLoadState('networkidle');
  
  // Scroll down gradually to trigger lazy loading
  for (let i = 0; i < 10; i++) {
    await page.mouse.wheel(0, 500);
    await page.waitForTimeout(300);
  }
  
  // Wait for lazy section to load and animation to complete
  await page.waitForTimeout(3000);
  
  // Find the canvas element (IndicatorsSection has a canvas)
  const canvas = page.locator('canvas').first();
  const canvasExists = await canvas.count() > 0;
  console.log('Canvas exists:', canvasExists);
  
  if (canvasExists) {
    const canvasBox = await canvas.boundingBox();
    console.log('Canvas dimensions:', canvasBox);
    
    // Check if canvas has been drawn to (non-blank)
    const hasContent = await canvas.evaluate((el: HTMLCanvasElement) => {
      const ctx = el.getContext('2d');
      if (!ctx) return false;
      const imageData = ctx.getImageData(0, 0, el.width, el.height);
      // Check if any pixel is non-transparent
      for (let i = 3; i < imageData.data.length; i += 4) {
        if (imageData.data[i] > 0) return true;
      }
      return false;
    });
    console.log('Canvas has content:', hasContent);
    expect(hasContent).toBe(true);
  } else {
    console.log('Canvas not found - checking page content');
    const pageContent = await page.content();
    console.log('Has IndicatorsSection text:', pageContent.includes('Trading Indicators'));
  }
  
  // Print relevant logs
  const relevantLogs = logs.filter(l => 
    l.toLowerCase().includes('indicator') || 
    l.toLowerCase().includes('canvas') || 
    l.toLowerCase().includes('chart')
  );
  if (relevantLogs.length > 0) {
    console.log('\nRelevant logs:', relevantLogs);
  }
  
  if (errors.length > 0) {
    console.log('\nErrors:', errors);
  }
});
