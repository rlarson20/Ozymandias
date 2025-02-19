// tests/unit-tests.js

describe('Firefox Extension', function() {
	describe('Logging', function() {
		it('should log the current site if enabled', function() {
			// Mock the extension settings to be enabled
			const settings = { loggingEnabled: true };

			// Mock the current tab
			const tab = { url: 'https://example.com' };

			// Mock the function that logs the site
			const logSite = jest.fn();

			// Call the function that logs the site
			require('../background-script.js').logCurrentSite(settings, tab, logSite);

			// Expect the logSite function to have been called with the current tab's URL
			expect(logSite).toHaveBeenCalledWith('https://example.com');
		});

		it('should not log the current site if disabled', function() {
			// Mock the extension settings to be disabled
			const settings = { loggingEnabled: false };

			// Mock the current tab
			const tab = { url: 'https://example.com' };

			// Mock the function that logs the site
			const logSite = jest.fn();

			// Call the function that logs the site
			require('../background-script.js').logCurrentSite(settings, tab, logSite);

			// Expect the logSite function to not have been called
			expect(logSite).not.toHaveBeenCalled();
		});
	});

	describe('Data Pushing', function() {
		it('should push data about the site to the main application', function() {
			// Mock the site data
			const siteData = { url: 'https://example.com', title: 'Example' };

			// Mock the function that pushes data to the main application
			const pushData = jest.fn();

			// Call the function that pushes data to the main application
			require('../background-script.js').pushSiteData(siteData, pushData);

			// Expect the pushData function to have been called with the site data
			expect(pushData).toHaveBeenCalledWith(siteData);
		});
	});

	describe('Comment Sending', function() {
		it('should send comments made to the database entry of the main application', function() {
			// Mock the comment
			const comment = 'This is a comment';

			// Mock the function that sends comments to the main application
			const sendComment = jest.fn();

			// Call the function that sends comments to the main application
			require('../background-script.js').sendComment(comment, sendComment);

			// Expect the sendComment function to have been called with the comment
			expect(sendComment).toHaveBeenCalledWith(comment);
		});
	});

	describe('Link Extraction', function() {
		it('should extract links from the page', function() {
			// Mock the page content
			const pageContent = `
        <html>
          <body>
            <a href="https://example.com/link1">Link 1</a>
            <a href="https://example.com/link2">Link 2</a>
          </body>
        </html>
      `;

			// Mock the function that extracts links from the page
			const extractLinks = jest.fn(() => [
				'https://example.com/link1',
				'https://example.com/link2',
			]);

			// Call the function that extracts links from the page
			const links = require('../background-script.js').extractLinks(pageContent, extractLinks);

			// Expect the extractLinks function to have been called with the page content
			expect(extractLinks).toHaveBeenCalledWith(pageContent);

			// Expect the links to have been extracted correctly
			expect(links).toEqual([
				'https://example.com/link1',
				'https://example.com/link2',
			]);
		});

		it('should send the list of links to the main application', function() {
			// Mock the list of links
			const links = [
				'https://example.com/link1',
				'https://example.com/link2',
			];

			// Mock the function that sends the list of links to the main application
			const sendLinks = jest.fn();

			// Call the function that sends the list of links to the main application
			require('../background-script.js').sendLinks(links, sendLinks);

			// Expect the sendLinks function to have been called with the list of links
			expect(sendLinks).toHaveBeenCalledWith(links);
		});
	});
});
