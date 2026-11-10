import Database from 'tauri-plugin-sql-api';
import { info } from 'tauri-plugin-log-api';

// L0: in-memory cache, lives with the translate window session.
// Keyed by text|source|target|instanceKey so different instances of the same
// service (e.g. two ollama instances with different models) never share results.
const memoryCache = new Map();

// L1: history.db doubles as the persistent cache. The index keeps the
// exact-match lookup fast as the table grows.
let indexReady = false;

// Dictionary services (Google/ECDICT) return objects which history stores as
// JSON strings. A translation result itself may also look like JSON, so only
// revive values that have the dictionary shape
const DICT_KEYS = ['pronunciations', 'explanations', 'associations', 'sentence'];

const buildKey = (text, source, target, instanceKey) => `${text}|${source}|${target}|${instanceKey}`;

function parseStoredResult(result) {
    if (typeof result !== 'string' || !result.trim().startsWith('{')) {
        return result;
    }
    try {
        const parsed = JSON.parse(result);
        if (parsed && typeof parsed === 'object' && DICT_KEYS.some((k) => k in parsed)) {
            return parsed;
        }
    } catch {}
    return result;
}

async function ensureIndex(db) {
    if (indexReady) return;
    await db.execute('CREATE INDEX IF NOT EXISTS idx_history_cache ON history(text, source, target, service)');
    indexReady = true;
}

// Returns the cached translation or undefined on miss
export async function getCachedTranslation(text, source, target, instanceKey, serviceName) {
    const key = buildKey(text, source, target, instanceKey);
    if (memoryCache.has(key)) {
        info(`[translate_cache] L0 hit: ${instanceKey}`);
        return memoryCache.get(key);
    }
    try {
        const db = await Database.load('sqlite:history.db');
        await ensureIndex(db);
        const rows = await db.select(
            'SELECT result FROM history WHERE text = $1 AND source = $2 AND target = $3 AND service = $4 ORDER BY id DESC LIMIT 1',
            [text, source, target, serviceName]
        );
        if (rows.length > 0) {
            const value = parseStoredResult(rows[0].result);
            memoryCache.set(key, value);
            info(`[translate_cache] L1 hit: ${serviceName}`);
            return value;
        }
    } catch (e) {
        // A cache failure must never break the translation itself
        info(`[translate_cache] L1 lookup failed: ${e}`);
    }
    return undefined;
}

export function setCachedTranslation(text, source, target, instanceKey, value) {
    memoryCache.set(buildKey(text, source, target, instanceKey), value);
}
