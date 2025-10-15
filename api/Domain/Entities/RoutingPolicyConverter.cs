using System.Text.Json;
using System.Text.Json.Serialization;

namespace ShortasProxyApi.Domain.Entities;

/// <summary>
/// Custom JSON converter for RoutingPolicy discriminated union
/// Matches Rust enum serialization format
/// </summary>
public class RoutingPolicyConverter : JsonConverter<RoutingPolicy>
{
    public override RoutingPolicy? Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
    {
        if (reader.TokenType == JsonTokenType.String)
        {
            var stringValue = reader.GetString();
            if (stringValue == "Basic")
                return new BasicPolicy();
            if (stringValue == "Mirroring")
                return new MirroringPolicy();
        }

        if (reader.TokenType != JsonTokenType.StartObject)
            throw new JsonException("Expected StartObject token");

        using var doc = JsonDocument.ParseValue(ref reader);
        var root = doc.RootElement;

        // Check for Conditional policy
        if (root.TryGetProperty("Conditional", out var conditionalElement))
        {
            var conditions = JsonSerializer.Deserialize<List<ConditionalRouting>>(
                conditionalElement.GetRawText(),
                options
            ) ?? new List<ConditionalRouting>();

            return new ConditionalPolicy { Conditions = conditions };
        }

        // Check for Challenge policy
        if (root.TryGetProperty("Challenge", out var challengeElement))
        {
            var challenge = JsonSerializer.Deserialize<ChallengeRouting>(
                challengeElement.GetRawText(),
                options
            );

            return new ChallengePolicy { Challenge = challenge };
        }

        // Check for File policy
        if (root.TryGetProperty("File", out var fileElement))
        {
            var file = JsonSerializer.Deserialize<FileRouting>(
                fileElement.GetRawText(),
                options
            );

            return new FilePolicy { File = file };
        }

        // Default to Basic
        return new BasicPolicy();
    }

    public override void Write(Utf8JsonWriter writer, RoutingPolicy value, JsonSerializerOptions options)
    {
        switch (value)
        {
            case BasicPolicy:
                writer.WriteStringValue("Basic");
                break;

            case MirroringPolicy:
                writer.WriteStringValue("Mirroring");
                break;

            case ConditionalPolicy conditional:
                writer.WriteStartObject();
                writer.WritePropertyName("Conditional");
                JsonSerializer.Serialize(writer, conditional.Conditions, options);
                writer.WriteEndObject();
                break;

            case ChallengePolicy challenge:
                writer.WriteStartObject();
                writer.WritePropertyName("Challenge");
                JsonSerializer.Serialize(writer, challenge.Challenge, options);
                writer.WriteEndObject();
                break;

            case FilePolicy file:
                writer.WriteStartObject();
                writer.WritePropertyName("File");
                JsonSerializer.Serialize(writer, file.File, options);
                writer.WriteEndObject();
                break;

            default:
                writer.WriteStringValue("Basic");
                break;
        }
    }
}
