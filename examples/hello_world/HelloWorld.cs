using Atipicial.SmartContract.Framework;
using Atipicial.SmartContract.Framework.Attributes;
using Atipicial.SmartContract.Framework.Services;

namespace Examples
{
    [ManifestExtra("Author", "atipicial-decompiler")]
    [ManifestExtra("Email", "dev@example.com")]
    [ManifestExtra("Description", "Minimal example contract for atipicial-decompiler walkthrough.")]
    public class HelloWorld : SmartContract
    {
        public static string Main()
        {
            return "hello, atipicial!";
        }

        public static void Notify(string message)
        {
            Runtime.Notify(message);
        }
    }
}
