use datafusion_expr::{Expr, LogicalPlan};
use std::collections::HashSet;
use datafusion_common::tree_node::{TreeNode, TreeNodeRecursion}; // Ensure TreeNode is used

pub struct Stats {
    pub query: String,
    pub logical_plan: LogicalPlan,
    pub num_of_joins: i32,
    pub num_of_predicates: i32,
    pub where_clauses: HashSet<Expr>,
    pub join_clauses: HashSet<Expr>,
}

impl Stats {
    pub fn new(sql_query: String, logical_plan: LogicalPlan) -> Self {
        Self {
            query: sql_query,
            logical_plan,
            num_of_joins: 0,
            num_of_predicates: 0,
            where_clauses: HashSet::new(),
            join_clauses: HashSet::new(),
        }
    }

    // Method to process the logical plan and gather WHERE and JOIN clauses
    pub fn process(&mut self) {
        self.logical_plan.apply(|node| {
            Self::process_where(node, &mut self.where_clauses);
            Self::process_join(node, &mut self.join_clauses);
            
            // Update counts
            self.num_of_predicates = self.where_clauses.len() as i32;
            self.num_of_joins = self.join_clauses.len() as i32;

            Ok(TreeNodeRecursion::Continue)
        }).unwrap();
    }

    // Method to process WHERE clauses
    fn process_where(node: &LogicalPlan, exprs: &mut HashSet<Expr>) {
        if let LogicalPlan::Filter(_filter) = node { // Use _filter to avoid warning
            node.apply_expressions(|expr| {
                process(expr, exprs);
                Ok(TreeNodeRecursion::Continue)
            }).unwrap();
        }
    }

    // Method to process JOIN clauses
    fn process_join(node: &LogicalPlan, exprs: &mut HashSet<Expr>) {
        if let LogicalPlan::Join(_join) = node { // Use _join to avoid warning
            node.apply_expressions(|expr| {
                process(expr, exprs);
                Ok(TreeNodeRecursion::Continue)
            }).unwrap();
        }
    }

    // Print stats method
    pub fn print_stats(&self) {
        println!("*******************************");
        println!("Number of joins: {}", self.num_of_joins);
        for expression in self.join_clauses.iter() {
            println!("Got: {}", expression);
            println!("Got: {:#?}", expression);
        }
        println!();
        println!("Number of predicates: {}", self.num_of_predicates);
        for expression in self.where_clauses.iter() {
            println!("Got: {}", expression);
        }
        println!("*******************************");
    }
}

// Helper function to process expressions recursively
fn process(expr: &Expr, exprs: &mut HashSet<Expr>) {
    if let Expr::BinaryExpr(binary_expr) = expr {
        let left = &binary_expr.left;
        let right = &binary_expr.right;
        if matches!(**left, Expr::Column(_)) {
            exprs.insert(expr.clone());
        } else if matches!(**left, Expr::BinaryExpr(_)) {
            process(left, exprs);
            process(right, exprs);
        } else {
            exprs.insert(*left.clone());
            process(right, exprs);
        }
    }
}
