import cloudscraper
import logging
import random
import time
import re
from bs4 import BeautifulSoup

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# List of modern User-Agents to rotate
USER_AGENTS = [
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
]

def deduplicate_content(text: str, max_ngram: int = 20) -> str:
    """
    Removes consecutive duplicate word sequences (n-grams).
    Example: 'Clear text Clear text' -> 'Clear text'
    """
    words = text.split()
    if not words:
        return ""
    
    result = []
    i = 0
    while i < len(words):
        found_dup = False
        for n in range(max_ngram, 0, -1):
            if i + 2*n <= len(words):
                seq1 = words[i:i+n]
                seq2 = words[i+n:i+2*n]
                if seq1 == seq2:
                    result.extend(seq1)
                    i += n
                    while i + n <= len(words) and words[i:i+n] == seq1:
                        i += n
                    found_dup = True
                    break
        
        if not found_dup:
            result.append(words[i])
            i += 1
            
    return " ".join(result)

def clean_html(html_content: str) -> str:
    """
    Cleans HTML content:
    - Removes all tags, scripts, and styles
    - Uses a pipe separator to identify block boundaries
    - Deduplicates identical blocks (phrases)
    - Normalizes whitespace
    - Removes consecutive duplicate word sequences
    """
    soup = BeautifulSoup(html_content, 'html.parser')
    
    # Remove script and style elements
    for script_or_style in soup(["script", "style"]):
        script_or_style.decompose()

    # Get text using a separator to identify boundaries between tags
    # This helps catch duplication where content is repeated in different tags
    raw_text = soup.get_text(separator='|')
    
    # Phrase-level deduplication (preserve order)
    blocks = [b.strip() for b in raw_text.split('|') if b.strip()]
    seen = set()
    recent_blocks = [] # To handle short repetitions
    unique_blocks = []
    
    for b in blocks:
        words_in_block = b.split()
        num_words = len(words_in_block)
        
        if num_words >= 2:
            if b not in seen:
                unique_blocks.append(b)
                seen.add(b)
        elif num_words == 1:
            # For 1-word blocks, only deduplicate if they appeared very recently
            if b not in recent_blocks:
                unique_blocks.append(b)
                recent_blocks.append(b)
                if len(recent_blocks) > 10:
                    recent_blocks.pop(0)

    text = " ".join(unique_blocks)
    
    # Normalize whitespace
    text = re.sub(r'\s+', ' ', text).strip()
    
    # Final pass: remove any remaining consecutive word sequences
    text = deduplicate_content(text)
    
    return text

def scrape_url(url: str) -> str:
    """
    Scrapes the given URL using cloudscraper with countermeasures:
    - Random User-Agent
    - Realistic headers
    - Random jitter (delay)
    
    Returns the HTML content as a string.
    Raises an exception if the request fails.
    """
    # 1. Random Jitter (1-3 seconds)
    delay = random.uniform(1.0, 3.0)
    logger.info(f"Scraping URL: {url} (Delay: {delay:.2f}s)")
    time.sleep(delay)

    # 2. Select Random Attributes
    user_agent = random.choice(USER_AGENTS)
    
    # 3. Configure Scraper
    scraper = cloudscraper.create_scraper(
        browser={
            'browser': 'chrome',
            'platform': 'windows',
            'desktop': True
        }
    )
    
    # 4. Set Headers
    headers = {
        "User-Agent": user_agent,
        "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
        "Accept-Language": "en-US,en;q=0.9,es;q=0.8",
        "Accept-Encoding": "gzip, deflate, br",
        "Referer": "https://www.google.com/",
        "Upgrade-Insecure-Requests": "1",
        "Sec-Fetch-Dest": "document",
        "Sec-Fetch-Mode": "navigate",
        "Sec-Fetch-Site": "cross-site",
        "Sec-Fetch-User": "?1",
        "Cache-Control": "max-age=0",
    }
    
    try:
        response = scraper.get(url, headers=headers)
        response.raise_for_status()
        return clean_html(response.text)
    except Exception as e:
        logger.error(f"Failed to scrape {url} with UA {user_agent}: {e}")
        raise
