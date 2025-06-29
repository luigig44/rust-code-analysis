use serde::Serialize;
use serde::ser::{SerializeStruct, Serializer};
use std::fmt;

use crate::checker::Checker;
use crate::macros::implement_metric_trait;
use crate::*;

/// The `Cyclomatic` metric.
#[derive(Debug, Clone)]
pub struct Stats {
    cyclomatic_sum: f64,
    cyclomatic: f64,
    n: usize,
    cyclomatic_max: f64,
    cyclomatic_min: f64,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            cyclomatic_sum: 0.,
            cyclomatic: 1.,
            n: 1,
            cyclomatic_max: 0.,
            cyclomatic_min: f64::MAX,
        }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("cyclomatic", 4)?;
        st.serialize_field("sum", &self.cyclomatic_sum())?;
        st.serialize_field("average", &self.cyclomatic_average())?;
        st.serialize_field("min", &self.cyclomatic_min())?;
        st.serialize_field("max", &self.cyclomatic_max())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sum: {}, average: {}, min: {}, max: {}",
            self.cyclomatic_sum(),
            self.cyclomatic_average(),
            self.cyclomatic_min(),
            self.cyclomatic_max()
        )
    }
}

impl Stats {
    /// Merges a second `Cyclomatic` metric into the first one
    pub fn merge(&mut self, other: &Stats) {
        //Calculate minimum and maximum values
        self.cyclomatic_max = self.cyclomatic_max.max(other.cyclomatic_max);
        self.cyclomatic_min = self.cyclomatic_min.min(other.cyclomatic_min);

        self.cyclomatic_sum += other.cyclomatic_sum;
        self.n += other.n;
    }

    /// Returns the `Cyclomatic` metric value
    pub fn cyclomatic(&self) -> f64 {
        self.cyclomatic
    }
    /// Returns the sum
    pub fn cyclomatic_sum(&self) -> f64 {
        self.cyclomatic_sum
    }

    /// Returns the `Cyclomatic` metric average value
    ///
    /// This value is computed dividing the `Cyclomatic` value for the
    /// number of spaces.
    pub fn cyclomatic_average(&self) -> f64 {
        self.cyclomatic_sum() / self.n as f64
    }
    /// Returns the `Cyclomatic` maximum value
    pub fn cyclomatic_max(&self) -> f64 {
        self.cyclomatic_max
    }
    /// Returns the `Cyclomatic` minimum value
    pub fn cyclomatic_min(&self) -> f64 {
        self.cyclomatic_min
    }
    #[inline(always)]
    pub(crate) fn compute_sum(&mut self) {
        self.cyclomatic_sum += self.cyclomatic;
    }
    #[inline(always)]
    pub(crate) fn compute_minmax(&mut self) {
        self.cyclomatic_max = self.cyclomatic_max.max(self.cyclomatic);
        self.cyclomatic_min = self.cyclomatic_min.min(self.cyclomatic);
        self.compute_sum();
    }
}

pub trait Cyclomatic
where
    Self: Checker,
{
    fn compute(node: &Node, stats: &mut Stats);
}

impl Cyclomatic for PythonCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Python::*;

        match node.kind_id().into() {
            If | Elif | For | While | Except | With | Assert | And | Or => {
                stats.cyclomatic += 1.;
            }
            Else => {
                if node.has_ancestors(
                    |node| matches!(node.kind_id().into(), ForStatement | WhileStatement),
                    |node| node.kind_id() == ElseClause,
                ) {
                    stats.cyclomatic += 1.;
                }
            }
            _ => {}
        }
    }
}

impl Cyclomatic for MozjsCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Mozjs::*;

        match node.kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for JavascriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Javascript::*;

        match node.kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for TypescriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Typescript::*;

        match node.kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for TsxCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Tsx::*;

        match node.kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for RustCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Rust::*;

        match node.kind_id().into() {
            If | For | While | Loop | MatchArm | MatchArm2 | TryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for CppCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Cpp::*;

        match node.kind_id().into() {
            If | For | While | Case | Catch | ConditionalExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for JavaCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Java::*;

        match node.kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

implement_metric_trait!(Cyclomatic, KotlinCode, PreprocCode, CcommentCode);

#[cfg(test)]
mod tests {
    use crate::tools::check_metrics;

    use super::*;

    #[test]
    fn python_simple_function() {
        check_metrics::<PythonParser>(
            "def f(a, b): # +2 (+1 unit space)
                if a and b:  # +2 (+1 and)
                   return 1
                if c and d: # +2 (+1 and)
                   return 1",
            "foo.py",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 6.0,
                      "average": 3.0,
                      "min": 1.0,
                      "max": 5.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_1_level_nesting() {
        check_metrics::<PythonParser>(
            "def f(a, b): # +2 (+1 unit space)
                if a:  # +1
                    for i in range(b):  # +1
                        return 1",
            "foo.py",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 4.0,
                      "average": 2.0,
                      "min": 1.0,
                      "max": 3.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_1_level_nesting() {
        check_metrics::<RustParser>(
            "fn f() { // +2 (+1 unit space)
                 if true { // +1
                     match true {
                         true => println!(\"test\"), // +1
                         false => println!(\"test\"), // +1
                     }
                 }
             }",
            "foo.rs",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 5.0,
                      "average": 2.5,
                      "min": 1.0,
                      "max": 4.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn c_switch() {
        check_metrics::<CppParser>(
            "void f() { // +2 (+1 unit space)
                 switch (1) {
                     case 1: // +1
                         printf(\"one\");
                         break;
                     case 2: // +1
                         printf(\"two\");
                         break;
                     case 3: // +1
                         printf(\"three\");
                         break;
                     default:
                         printf(\"all\");
                         break;
                 }
             }",
            "foo.c",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 5.0,
                      "average": 2.5,
                      "min": 1.0,
                      "max": 4.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn c_real_function() {
        check_metrics::<CppParser>(
            "int sumOfPrimes(int max) { // +2 (+1 unit space)
                 int total = 0;
                 OUT: for (int i = 1; i <= max; ++i) { // +1
                   for (int j = 2; j < i; ++j) { // +1
                       if (i % j == 0) { // +1
                          continue OUT;
                       }
                   }
                   total += i;
                 }
                 return total;
            }",
            "foo.c",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 5.0,
                      "average": 2.5,
                      "min": 1.0,
                      "max": 4.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn c_unit_before() {
        check_metrics::<CppParser>(
            "
            int a=42;
            if(a==42) //+2(+1 unit space)
            {

            }
            if(a==34) //+1
            {

            }
            int sumOfPrimes(int max) { // +1
                 int total = 0;
                 OUT: for (int i = 1; i <= max; ++i) { // +1
                   for (int j = 2; j < i; ++j) { // +1
                       if (i % j == 0) { // +1
                          continue OUT;
                       }
                   }
                   total += i;
                 }
                 return total;
            }",
            "foo.c",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 7.0,
                      "average": 3.5,
                      "min": 3.0,
                      "max": 4.0
                    }"###
                );
            },
        );
    }

    /// Test to handle the case of min and max when merge happen before the final value of one module are set.
    /// In this case the min value should be 3 because the unit space has 2 branches and a complexity of 3
    /// while the function sumOfPrimes has a complexity of 4.
    #[test]
    fn c_unit_after() {
        check_metrics::<CppParser>(
            "
            int sumOfPrimes(int max) { // +1
                 int total = 0;
                 OUT: for (int i = 1; i <= max; ++i) { // +1
                   for (int j = 2; j < i; ++j) { // +1
                       if (i % j == 0) { // +1
                          continue OUT;
                       }
                   }
                   total += i;
                 }
                 return total;
            }

            int a=42;
            if(a==42) //+2(+1 unit space)
            {

            }
            if(a==34) //+1
            {

            }",
            "foo.c",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 7.0,
                      "average": 3.5,
                      "min": 3.0,
                      "max": 4.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_simple_class() {
        check_metrics::<JavaParser>(
            "
            public class Example { // +2 (+1 unit space)
                int a = 10;
                boolean b = (a > 5) ? true : false; // +1
                boolean c = b && true; // +1

                public void m1() { // +1
                    if (a % 2 == 0) { // +1
                        b = b || c; // +1
                    }
                }
                public void m2() { // +1
                    while (a > 3) { // +1
                        m1();
                        a--;
                    }
                }
            }",
            "foo.java",
            |metric| {
                // nspace = 4 (unit, class and 2 methods)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 9.0,
                      "average": 2.25,
                      "min": 1.0,
                      "max": 3.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_real_class() {
        check_metrics::<JavaParser>(
            "
            public class Matrix { // +2 (+1 unit space)
                private int[][] m = new int[5][5];

                public void init() { // +1
                    for (int i = 0; i < m.length; i++) { // +1
                        for (int j = 0; j < m[i].length; j++) { // +1
                            m[i][j] = i * j;
                        }
                    }
                }
                public int compute(int i, int j) { // +1
                    try {
                        return m[i][j] / m[j][i];
                    } catch (ArithmeticException e) { // +1
                        return -1;
                    } catch (ArrayIndexOutOfBoundsException e) { // +1
                        return -2;
                    }
                }
                public void print(int result) { // +1
                    switch (result) {
                        case -1: // +1
                            System.out.println(\"Division by zero\");
                            break;
                        case -2: // +1
                            System.out.println(\"Wrong index number\");
                            break;
                        default:
                            System.out.println(\"The result is \" + result);
                    }
                }
            }",
            "foo.java",
            |metric| {
                // nspace = 5 (unit, class and 3 methods)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 11.0,
                      "average": 2.2,
                      "min": 1.0,
                      "max": 3.0
                    }"###
                );
            },
        );
    }

    // As reported here:
    // https://github.com/sebastianbergmann/php-code-coverage/issues/607
    // An anonymous class declaration is not considered when computing the Cyclomatic Complexity metric for Java
    // Only the complexity of the anonymous class content is considered for the computation
    #[test]
    fn java_anonymous_class() {
        check_metrics::<JavaParser>(
            "
            abstract class A { // +2 (+1 unit space)
                public abstract boolean m1(int n); // +1
                public abstract boolean m2(int n); // +1
            }
            public class B { // +1

                public void test() { // +1
                    A a = new A() {
                        public boolean m1(int n) { // +1
                            if (n % 2 == 0) { // +1
                                return true;
                            }
                            return false;
                        }
                        public boolean m2(int n) { // +1
                            if (n % 5 == 0) { // +1
                                return true;
                            }
                            return false;
                        }
                    };
                }
            }",
            "foo.java",
            |metric| {
                // nspace = 8 (unit, 2 classes and 5 methods)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 10.0,
                      "average": 1.25,
                      "min": 1.0,
                      "max": 2.0
                    }"###
                );
            },
        );
    }
}
