# 查询示例

> 本文件收录常用的开发日志查询示例。

---

## 基础查询

### 1. 获取最近 N 天的日志

```bash
node .claude/skills/dev-log/scripts/search-logs.js --list --days=7
node .claude/skills/dev-log/scripts/search-logs.js --list --days=30
```

---

### 2. 查看指定日期的日志

```bash
node .claude/skills/dev-log/scripts/search-logs.js --date=2026-03-11
```

---

### 3. 语义搜索

```bash
node .claude/skills/dev-log/scripts/search-logs.js "密钥修改"
node .claude/skills/dev-log/scripts/search-logs.js "积分计算 Bug"
```

**带数量限制:**
```bash
node .claude/skills/dev-log/scripts/search-logs.js "支付" --limit=20
```

---

## 直接调用向量库 API (Node.js)

```javascript
const vectorStore = require('./vector-store');

// 搜索
const results = await vectorStore.searchLogs('密钥修改', { limit: 5 });

// 添加日志
await vectorStore.addLog('unique_id', {}, '修复积分计算错误');

// 删除
await vectorStore.deleteLog('unique_id');

// 统计
const stats = await vectorStore.getStats();
console.log(`向量库中有 ${stats.count} 条日志`);
```

---

## 最佳实践

1. **先语义搜索**：用自然语言描述你想查找的内容
2. **每日记录**：每次代码变更后对 AI 说 "record log" 记录变更
3. **定期回顾**：每周对 AI 说 "查看最近 7 天的开发日志"