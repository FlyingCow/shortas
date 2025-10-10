using ShortasProxyApi.Domain.Entities;

namespace ShortasProxyApi.Domain.Interfaces;

public interface IOutboxRepository
{
    Task AddAsync(OutboxMessage message);
    Task AddRangeAsync(IEnumerable<OutboxMessage> messages);
    Task<List<OutboxMessage>> GetPendingMessagesAsync(int batchSize = 10);
    Task UpdateAsync(OutboxMessage message);
    Task<int> SaveChangesAsync();
}
