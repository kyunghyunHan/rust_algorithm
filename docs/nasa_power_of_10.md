# NASA Power of 10 Coding Rules (Summary & Explanation)

1. **All control flow must be explicit and visible.**  
   Example: Do not use `goto`. Use clear loops and conditionals.  

2. **All loops must be finite.**  
   Infinite loops are forbidden. Every loop must have a clear termination condition.  

3. **All assignments must be explicit.**  
   No implicit type conversions or hidden pointer arithmetic. Assign values directly and clearly.  

4. **All data must be explicitly initialized.**  
   Every declared variable must be given an initial value.  

5. **Pointers must be used in a restricted way.**  
   Avoid pointer arithmetic and multiple levels of indirection. If pointers are necessary, use them with extreme caution.  

6. **Functions must be short and simple.**  
   Typically, functions should not exceed one page (about 50 lines) and should perform only a single task.  

7. **Header files must contain declarations only.**  
   Do not define variables in headers. Only include function and type declarations.  

8. **Code must remain within the limits of human understanding.**  
   Avoid overly complex expressions or excessive abstraction. Code should be understandable by human reviewers.  

9. **Source code must be verifiable by static analysis tools.**  
   Tools like Lint or Coverity should be able to check the code without producing errors or warnings.  

10. **Compiler warnings must be reduced to zero.**  
   All compiler warnings must be fixed; none should be ignored.  
