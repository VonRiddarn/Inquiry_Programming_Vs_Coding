namespace PirateTranslatorCsharp;

class Program
{
	
	const string VOWELS = "aouåeiyäö";
	static string answer = String.Empty;
	
	static bool IsVowel(char c) => VOWELS.ToLower().Contains(Char.ToLower(c));
	
    static void Main(string[] args)
    {
		
		while(true)
		{
			Console.Clear();
			Console.WriteLine("Yarr ohoy matey! Welocme to me pirate translator!");
			Console.WriteLine("Do be typey '.exit' to leave me program! Yarr");
			Console.Write("Input: ");
			
			answer = Console.ReadLine()!;
			
			if(answer.Length > 0)
			{
				if(answer.ToLower() == ".exit")
				{
					break;
				}
				
				Console.WriteLine(Translate(answer));
			}
			else
			{
				Console.WriteLine("Oy, put something in me translator!");	
			}
			
			// Pause the current itteration.
			Console.ReadLine();
		}
		
		Console.WriteLine("Good bye, matey!");
		
    }
	
	
	static string Translate(string textToTranslate)
	{
		string outPutString = String.Empty;
		
		foreach(char c in textToTranslate)
		{
			// Early return pattern
			if(IsVowel(c) || !Char.IsLetter(c))
			{
				outPutString += c;
				continue;
			}
			
			// This is a consonant!
			outPutString += Char.IsUpper(c) ? $"{c}O{c}" : $"{c}o{c}";
		}
		
		return outPutString;
	}
}
