using Microsoft.EntityFrameworkCore;
using ShortasProxyApi.Domain.Entities;
using ShortasProxyApi.Domain.Interfaces;
using ShortasProxyApi.Infrastructure.Data;

namespace ShortasProxyApi.Infrastructure.Repositories;

public class OutboxRepository : IOutboxRepository
{
    private readonly ApplicationDbContext _context;

    public OutboxRepository(ApplicationDbContext context)
    {
        _context = context;
    }

    public async Task AddAsync(OutboxMessage message)
    {
        await _context.OutboxMessages.AddAsync(message);
    }

    public async Task AddRangeAsync(IEnumerable<OutboxMessage> messages)
    {
        await _context.OutboxMessages.AddRangeAsync(messages);
    }

    public async Task<List<OutboxMessage>> GetPendingMessagesAsync(int batchSize = 10)
    {
        var now = DateTime.UtcNow;

        return await _context.OutboxMessages
            .Where(m => m.Status == OutboxMessageStatus.Pending &&
                       (m.NextRetryAt == null || m.NextRetryAt <= now))
            .OrderBy(m => m.CreatedAt)
            .Take(batchSize)
            .ToListAsync();
    }

    public Task UpdateAsync(OutboxMessage message)
    {
        _context.OutboxMessages.Update(message);
        return Task.CompletedTask;
    }

    public async Task<int> SaveChangesAsync()
    {
        return await _context.SaveChangesAsync();
    }
}
