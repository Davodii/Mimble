using System.Globalization;
using Mimble.Exceptions;

namespace Mimble.Values;

public class NumberValue : ConstantValue
{
    public NumberValue(double value) : base(ValueType.Number)
    {
        Value = value;
    }
    
    public double AsNumber()
    {
        return (double)Value;
    }
    
    public int AsInteger()
    {
        if (!IsNumber(this) || (double)Value % 1 != 0)
        {
            // Not a whole number
            throw new ConversionException(this, ValueType.Number);
        }
        
        return (int)(double)Value;
    }
    
    public override string ToString()
    {
        return AsNumber().ToString(CultureInfo.InvariantCulture);
    }

    public override bool Equals(object? obj)
    {
        if (obj == null || obj.GetType() != this.GetType()) return false;
        return Math.Abs(((NumberValue)obj).AsNumber() - AsNumber()) < 0.001f;
    }
}