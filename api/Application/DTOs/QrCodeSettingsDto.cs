namespace ShortasProxyApi.Application.DTOs;

public class QrCodeSettingsDto
{
    public string DotStyle { get; set; } = "rounded";
    public string FgColor { get; set; } = "#000000";
    public string BgColor { get; set; } = "#ffffff";
    public int Size { get; set; } = 280;
    public string? CenterImageUrl { get; set; }
}

public class PresignedUrlResponseDto
{
    public string Url { get; set; } = string.Empty;
    public string Key { get; set; } = string.Empty;
}

public class PresignedUploadRequestDto
{
    public string? ContentType { get; set; }
}
