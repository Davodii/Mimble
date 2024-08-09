using System.Globalization;
using VMProject.Exceptions;

namespace VMProject;

public class NumberValue : ConstantValue
{
    public NumberValue(double value) : base(ValueType.Number)
    {
        _value = value;
    }
    
    public double AsNumber()
    {
        return (double)_value;
    }
    
    public int AsInteger()
    {
        if (!IsNumber(this) || (double)_value % 1 != 0)
        {
            // Not a whole number
            throw new ConversionException(this, ValueType.Number);
        }
        
        return (int)(double)_value;
    }
    
    public override string ToString()
    {
        return AsNumber().ToString(CultureInfo.InvariantCulture);
    }
}