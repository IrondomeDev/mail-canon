using IronDome.MailCanon;
using System.Text;

static void Check(bool condition, string name)
{
    if (!condition) throw new Exception(name);
}

int count = 0;
var utf8 = new UTF8Encoding(false, true);
foreach (string row in File.ReadLines(Path.Combine(AppContext.BaseDirectory, "vectors.tsv")))
{
    if (row.StartsWith('#')) continue;
    string[] fields = row.Split('\t');
    string raw = utf8.GetString(Convert.FromHexString(fields[1]));
    if (fields[2] == "-")
    {
        Check(!EmailNormalizer.TryNormalize(raw, out _), fields[0]);
        try { EmailNormalizer.Normalize(raw); throw new Exception(fields[0] + ": accepted invalid input"); }
        catch (InvalidEmailException) { }
        try { EmailNormalizer.Canonicalize(raw); throw new Exception(fields[0] + ": canonicalize accepted invalid input"); }
        catch (InvalidEmailException) { }
    }
    else
    {
        var expected = new EmailIdentity(fields[2], fields[3], fields[4]);
        Check(EmailNormalizer.Normalize(raw) == expected, fields[0]);
        Check(EmailNormalizer.TryNormalize(raw, out var value) && value == expected, fields[0]);
        Check(EmailNormalizer.Canonicalize(raw) == fields[3], fields[0]);
        Check(EmailNormalizer.Canonicalize(fields[3]) == fields[3], fields[0] + ": not idempotent");
    }
    count++;
}
Check(!EmailNormalizer.TryNormalize(null, out _), "null");
foreach (string row in File.ReadLines(Path.Combine(AppContext.BaseDirectory, "strict-vectors.tsv")))
{
    if (row.StartsWith('#')) continue;
    string[] fields = row.Split('\t');
    string raw = utf8.GetString(Convert.FromHexString(fields[1]));
    if (fields[2] == "-")
    {
        Check(!EmailNormalizer.TryNormalizeStrict(raw, out _), fields[0]);
        try { EmailNormalizer.NormalizeStrict(raw); throw new Exception(fields[0] + ": strict accepted invalid input"); }
        catch (InvalidEmailException) { }
        try { EmailNormalizer.CanonicalizeStrict(raw); throw new Exception(fields[0] + ": strict canonicalize accepted invalid input"); }
        catch (InvalidEmailException) { }
    }
    else
    {
        var expected = new EmailIdentity(fields[2], fields[3], fields[4]);
        Check(EmailNormalizer.NormalizeStrict(raw) == expected, fields[0]);
        Check(EmailNormalizer.TryNormalizeStrict(raw, out var value) && value == expected, fields[0]);
        Check(EmailNormalizer.CanonicalizeStrict(raw) == fields[3], fields[0]);
        Check(EmailNormalizer.CanonicalizeStrict(fields[3]) == fields[3], fields[0] + ": strict not idempotent");
    }
}
Check(!EmailNormalizer.TryNormalizeStrict(null, out _), "strict null");
try { EmailNormalizer.NormalizeStrict(null!); throw new Exception("strict accepted null"); }
catch (ArgumentNullException) { }
try { EmailNormalizer.Normalize(null!); throw new Exception("accepted null"); }
catch (ArgumentNullException) { }
foreach (string raw in new[] { "\ud800@example.com", "name \udfff <u@example.com>" })
{
    Check(!EmailNormalizer.TryNormalize(raw, out _), "ill-formed UTF-16");
    Check(!EmailNormalizer.TryNormalizeStrict(raw, out _), "strict ill-formed UTF-16");
}
Console.WriteLine($"Passed {count} vectors, null checks, and malformed UTF-16 checks.");

foreach (string raw in new[] { "ma\u0131lto:u@example.com", "ma\u0130lto:u@example.com" })
    Check(!EmailNormalizer.TryNormalize(raw, out _), "non-ASCII mailto prefix");
