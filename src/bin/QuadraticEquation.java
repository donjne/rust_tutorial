import java.util.Scanner; 
import java.lang.Math; 
public class QuadraticEquation { 
 public static void main(String[] args) { 
 Scanner scanner = new Scanner(System.in); 
 System.out.println("Enter coefficient a:"); 
 double a = scanner.nextDouble(); 
 System.out.println("Enter coefficient b:"); 
 double b = scanner.nextDouble(); 
 System.out.println("Enter coefficient c:"); 
 double c = scanner.nextDouble(); 
 double d = b * b - 4 * a * c; 
 if (d > 0) { 
 double x1 = (-b + Math.sqrt(d)) / (2 * a); 
 double x2 = (-b - Math.sqrt(d)) / (2 * a); 
 System.out.println("Roots are real and different:"); 
 System.out.println("x1 = " + x1); 
 System.out.println("x2 = " + x2); 
 } else if (d == 0) { 
 double x = -b / (2 * a); 
 System.out.println("Roots are real and same:"); 
 System.out.println("x1 = x2 = " + x); 
 } else { 
 double realPart = -b / (2 * a); 
 double imaginaryPart = Math.sqrt(-d) / (2 * a); 
 System.out.println("Roots are complex and different:"); 
 System.out.println("x1 = " + realPart + " + " + imaginaryPart + "i"); 
 System.out.println("x2 = " + realPart + " - " + imaginaryPart + "i"); 
 } 
 } 
} 