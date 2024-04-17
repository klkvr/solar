//! Constant and mutable AST visitor trait definitions.

use crate::ast::*;
use sulk_interface::{Ident, Span};
use sulk_macros::declare_visitors;

declare_visitors! {
    /// AST traversal.
    pub trait Visit VisitMut <'ast> {
        fn visit_source_unit(&mut self, source_unit: &'ast #mut SourceUnit) {
            let SourceUnit { items } = source_unit;
            for item in items {
                self.visit_item #_mut(item);
            }
        }

        fn visit_item(&mut self, item: &'ast #mut Item) {
            let Item { docs, span, kind } = item;
            self.visit_span #_mut(span);
            self.visit_doc_comments #_mut(docs);
            match kind {
                ItemKind::Pragma(item) => self.visit_pragma_directive #_mut(item),
                ItemKind::Import(item) => self.visit_import_directive #_mut(item),
                ItemKind::Using(item) => self.visit_using_directive #_mut(item),
                ItemKind::Contract(item) => self.visit_item_contract #_mut(item),
                ItemKind::Function(item) => self.visit_item_function #_mut(item),
                ItemKind::Variable(item) => self.visit_variable_definition #_mut(item),
                ItemKind::Struct(item) => self.visit_item_struct #_mut(item),
                ItemKind::Enum(item) => self.visit_item_enum #_mut(item),
                ItemKind::Udvt(item) => self.visit_item_udvt #_mut(item),
                ItemKind::Error(item) => self.visit_item_error #_mut(item),
                ItemKind::Event(item) => self.visit_item_event #_mut(item),
            }
        }

        fn visit_pragma_directive(&mut self, pragma: &'ast #mut PragmaDirective) {
            // noop by default.
            let PragmaDirective { tokens: _ } = pragma;
        }

        fn visit_import_directive(&mut self, import: &'ast #mut ImportDirective) {
            let ImportDirective { path, items } = import;
            let _ = path; // TODO: ?
            match items {
                ImportItems::Plain(alias) => {
                    if let Some(alias) = alias {
                        self.visit_ident #_mut(alias);
                    }
                }
                ImportItems::Aliases(paths) => {
                    for (import, alias) in paths {
                        self.visit_ident #_mut(import);
                        if let Some(alias) = alias {
                            self.visit_ident #_mut(alias);
                        }
                    }
                }
                ImportItems::Glob(alias) => {
                    if let Some(alias) = alias {
                        self.visit_ident #_mut(alias);
                    }
                }
            }
        }

        fn visit_using_directive(&mut self, using: &'ast #mut UsingDirective) {
            let UsingDirective { list, ty, global: _ } = using;
            match list {
                UsingList::Single(path) => {
                    self.visit_path #_mut(path);
                }
                UsingList::Multiple(paths) => {
                    for (path, _) in paths {
                        self.visit_path #_mut(path);
                    }
                }
            }
            if let Some(ty) = ty {
                self.visit_ty #_mut(ty);
            }
        }

        fn visit_item_contract(&mut self, contract: &'ast #mut ItemContract) {
            let ItemContract { kind: _, name, inheritance, body } = contract;
            self.visit_ident #_mut(name);
            for modifier in inheritance {
                self.visit_modifier #_mut(modifier);
            }
            for item in body {
                self.visit_item #_mut(item);
            }
        }

        fn visit_item_function(&mut self, function: &'ast #mut ItemFunction) {
            let ItemFunction { kind: _, header, body } = function;
            self.visit_function_header #_mut(header);
            if let Some(body) = body {
                self.visit_block #_mut(body);
            }
        }

        fn visit_item_struct(&mut self, strukt: &'ast #mut ItemStruct) {
            let ItemStruct { name, fields } = strukt;
            self.visit_ident #_mut(name);
            for field in fields {
                self.visit_variable_definition #_mut(field);
            }
        }

        fn visit_item_enum(&mut self, enum_: &'ast #mut ItemEnum) {
            let ItemEnum { name, variants } = enum_;
            self.visit_ident #_mut(name);
            for variant in variants {
                self.visit_ident #_mut(variant);
            }
        }

        fn visit_item_udvt(&mut self, udvt: &'ast #mut ItemUdvt) {
            let ItemUdvt { name, ty } = udvt;
            self.visit_ident #_mut(name);
            self.visit_ty #_mut(ty);
        }

        fn visit_item_error(&mut self, error: &'ast #mut ItemError) {
            let ItemError { name, parameters } = error;
            self.visit_ident #_mut(name);
            self.visit_parameter_list #_mut(parameters);
        }

        fn visit_item_event(&mut self, event: &'ast #mut ItemEvent) {
            let ItemEvent { name, parameters, anonymous: _ } = event;
            self.visit_ident #_mut(name);
            self.visit_parameter_list #_mut(parameters);
        }

        fn visit_variable_definition(&mut self, var: &'ast #mut VariableDefinition) {
            let VariableDefinition {
                ty,
                visibility: _,
                mutability: _,
                data_location: _,
                override_: _,
                indexed: _,
                name,
                initializer,
            } = var;
            self.visit_ty #_mut(ty);
            if let Some(name) = name {
                self.visit_ident #_mut(name);
            }
            if let Some(initializer) = initializer {
                self.visit_expr #_mut(initializer);
            }
        }

        fn visit_ty(&mut self, ty: &'ast #mut Ty) {
            let Ty { span, kind } = ty;
            self.visit_span #_mut(span);
            match kind {
                TyKind::Address(_payable) => {}
                TyKind::Bool => {}
                TyKind::String => {}
                TyKind::Bytes => {}
                TyKind::Fixed(_m, _n) => {}
                TyKind::UFixed(_m, _n) => {}
                TyKind::Int(_n) => {}
                TyKind::UInt(_n) => {}
                TyKind::FixedBytes(_n) => {}
                TyKind::Array(array) => {
                    let TypeArray { element, size: _ } = &#mut **array;
                    self.visit_ty #_mut(element);
                }
                TyKind::Function(function) => {
                    self.visit_function_header #_mut(function);
                }
                TyKind::Mapping(mapping) => {
                    let TypeMapping { key, key_name, value, value_name } = &#mut **mapping;
                    self.visit_ty #_mut(key);
                    if let Some(key_name) = key_name {
                        self.visit_ident #_mut(key_name);
                    }
                    self.visit_ty #_mut(value);
                    if let Some(value_name) = value_name {
                        self.visit_ident #_mut(value_name);
                    }
                }
                TyKind::Custom(path) => {
                    self.visit_path #_mut(path);
                }
            }
        }

        fn visit_function_header(&mut self, header: &'ast #mut FunctionHeader) {
            let FunctionHeader {
                name,
                parameters,
                visibility: _,
                state_mutability: _,
                modifiers,
                virtual_: _,
                override_: _,
                returns,
            } = header;
            if let Some(name) = name {
                self.visit_ident #_mut(name);
            }
            self.visit_parameter_list #_mut(parameters);
            for modifier in modifiers {
                self.visit_modifier #_mut(modifier);
            }
            self.visit_parameter_list #_mut(returns);
        }

        fn visit_modifier(&mut self, modifier: &'ast #mut Modifier) {
            let Modifier { name, arguments } = modifier;
            self.visit_path #_mut(name);
            self.visit_call_args #_mut(arguments);
        }

        fn visit_call_args(&mut self, args: &'ast #mut CallArgs) {
            match args {
                CallArgs::Named(named) => {
                    self.visit_named_args #_mut(named);
                }
                CallArgs::Unnamed(unnamed) => {
                    for arg in unnamed {
                        self.visit_expr #_mut(arg);
                    }
                }
            }
        }

        fn visit_named_args(&mut self, args: &'ast #mut NamedArgList) {
            for NamedArg { name, value } in args {
                self.visit_ident #_mut(name);
                self.visit_expr #_mut(value);
            }
        }

        fn visit_stmt(&mut self, stmt: &'ast #mut Stmt) {
            let Stmt { docs, span, kind } = stmt;
            self.visit_doc_comments #_mut(docs);
            self.visit_span #_mut(span);
            match kind {
                StmtKind::Assembly(assembly) => {
                    self.visit_stmt_assembly #_mut(assembly);
                }
                StmtKind::DeclSingle(var) => {
                    self.visit_variable_definition #_mut(var);
                }
                StmtKind::DeclMulti(vars, expr) => {
                    for var in vars {
                        if let Some(var) = var {
                            self.visit_variable_definition #_mut(var);
                        }
                    }
                    self.visit_expr #_mut(expr);
                }
                StmtKind::Block(block) => {
                    self.visit_block #_mut(block);
                }
                StmtKind::Break => {}
                StmtKind::Continue => {}
                StmtKind::DoWhile(block, expr) => {
                    self.visit_block #_mut(block);
                    self.visit_expr #_mut(expr);
                }
                StmtKind::Emit(path, args) => {
                    self.visit_path #_mut(path);
                    self.visit_call_args #_mut(args);
                }
                StmtKind::Expr(expr) => {
                    self.visit_expr #_mut(expr);
                }
                StmtKind::For { init, cond, next, body } => {
                    if let Some(init) = init {
                        self.visit_stmt #_mut(init);
                    }
                    if let Some(cond) = cond {
                        self.visit_expr #_mut(cond);
                    }
                    if let Some(next) = next {
                        self.visit_expr #_mut(next);
                    }
                    self.visit_stmt #_mut(body);
                }
                StmtKind::If(cond, then, else_) => {
                    self.visit_expr #_mut(cond);
                    self.visit_stmt #_mut(then);
                    if let Some(else_) = else_ {
                        self.visit_stmt #_mut(else_);
                    }
                }
                StmtKind::Return(expr) => {
                    if let Some(expr) = expr {
                        self.visit_expr #_mut(expr);
                    }
                }
                StmtKind::Revert(path, args) => {
                    self.visit_path #_mut(path);
                    self.visit_call_args #_mut(args);
                }
                StmtKind::Try(try_) => {
                    self.visit_stmt_try #_mut(try_);
                }
                StmtKind::UncheckedBlock(block) => {
                    self.visit_block #_mut(block);
                }
                StmtKind::While(cond, block) => {
                    self.visit_expr #_mut(cond);
                    self.visit_stmt #_mut(block);
                }
            }
        }

        fn visit_stmt_assembly(&mut self, assembly: &'ast #mut StmtAssembly) {
            let StmtAssembly { dialect: _, flags: _, block } = assembly;
            self.visit_yul_block #_mut(block);
        }

        fn visit_stmt_try(&mut self, try_: &'ast #mut StmtTry) {
            let StmtTry { expr, returns, block, catch } = try_;
            self.visit_expr #_mut(expr);
            self.visit_parameter_list #_mut(returns);
            self.visit_block #_mut(block);
            for catch in catch {
                self.visit_catch_clause #_mut(catch);
            }
        }

        fn visit_catch_clause(&mut self, catch: &'ast #mut CatchClause) {
            let CatchClause { name, args, block } = catch;
            if let Some(name) = name {
                self.visit_ident #_mut(name);
            }
            self.visit_parameter_list #_mut(args);
            self.visit_block #_mut(block);
        }

        fn visit_block(&mut self, block: &'ast #mut Block) {
            for stmt in block {
                self.visit_stmt #_mut(stmt);
            }
        }

        fn visit_expr(&mut self, expr: &'ast #mut Expr) {
            let Expr { span, kind } = expr;
            self.visit_span #_mut(span);
            match kind {
                ExprKind::Array(exprs) => {
                    for expr in exprs {
                        self.visit_expr #_mut(expr);
                    }
                }
                ExprKind::Assign(lhs, _op, rhs) => {
                    self.visit_expr #_mut(lhs);
                    self.visit_expr #_mut(rhs);
                }
                ExprKind::Binary(lhs, _op, rhs) => {
                    self.visit_expr #_mut(lhs);
                    self.visit_expr #_mut(rhs);
                }
                ExprKind::Call(lhs, args) => {
                    self.visit_expr #_mut(lhs);
                    self.visit_call_args #_mut(args);
                }
                ExprKind::CallOptions(lhs, args) => {
                    self.visit_expr #_mut(lhs);
                    self.visit_named_args #_mut(args);
                }
                ExprKind::Delete(expr) => {
                    self.visit_expr #_mut(expr);
                }
                ExprKind::Ident(ident) => {
                    self.visit_ident #_mut(ident);
                }
                ExprKind::Index(lhs, kind) => {
                    self.visit_expr #_mut(lhs);
                    match kind {
                        IndexKind::Index(expr) => {
                            if let Some(expr) = expr {
                                self.visit_expr #_mut(expr);
                            }
                        }
                        IndexKind::Range(start, end) => {
                            if let Some(start) = start {
                                self.visit_expr #_mut(start);
                            }
                            if let Some(end) = end {
                                self.visit_expr #_mut(end);
                            }
                        }
                    }
                }
                ExprKind::Lit(lit, _sub) => {
                    self.visit_lit #_mut(lit);
                }
                ExprKind::Member(expr, member) => {
                    self.visit_expr #_mut(expr);
                    self.visit_ident #_mut(member);
                }
                ExprKind::New(ty) => {
                    self.visit_ty #_mut(ty);
                }
                ExprKind::Payable(args) => {
                    self.visit_call_args #_mut(args);
                }
                ExprKind::Ternary(cond, true_, false_) => {
                    self.visit_expr #_mut(cond);
                    self.visit_expr #_mut(true_);
                    self.visit_expr #_mut(false_);
                }
                ExprKind::Tuple(exprs) => {
                    for expr in exprs {
                        if let Some(expr) = expr {
                            self.visit_expr #_mut(expr);
                        }
                    }
                }
                ExprKind::TypeCall(ty) => {
                    self.visit_ty #_mut(ty);
                }
                ExprKind::Type(ty) => {
                    self.visit_ty #_mut(ty);
                }
                ExprKind::Unary(_op, expr) => {
                    self.visit_expr #_mut(expr);
                }
            }
        }

        fn visit_parameter_list(&mut self, list: &'ast #mut ParameterList) {
            for param in list {
                self.visit_variable_definition #_mut(param);
            }
        }

        fn visit_lit(&mut self, lit: &'ast #mut Lit) {
            let Lit { span, symbol: _, kind: _ } = lit;
            self.visit_span #_mut(span);
        }

        fn visit_yul_stmt(&mut self, stmt: &'ast #mut yul::Stmt) {
            let yul::Stmt { docs, span, kind } = stmt;
            self.visit_doc_comments #_mut(docs);
            self.visit_span #_mut(span);
            match kind {
                yul::StmtKind::Block(block) => {
                    self.visit_yul_block #_mut(block);
                }
                yul::StmtKind::AssignSingle(path, expr) => {
                    self.visit_path #_mut(path);
                    self.visit_yul_expr #_mut(expr);
                }
                yul::StmtKind::AssignMulti(paths, call) => {
                    for path in paths {
                        self.visit_path #_mut(path);
                    }
                    self.visit_yul_expr_call #_mut(call);
                }
                yul::StmtKind::Expr(call) => {
                    self.visit_yul_expr_call #_mut(call);
                }
                yul::StmtKind::If(expr, block) => {
                    self.visit_yul_expr #_mut(expr);
                    self.visit_yul_block #_mut(block);
                }
                yul::StmtKind::For { init, cond, step, body } => {
                    self.visit_yul_block #_mut(init);
                    self.visit_yul_expr #_mut(cond);
                    self.visit_yul_block #_mut(step);
                    self.visit_yul_block #_mut(body);
                }
                yul::StmtKind::Switch(switch) => {
                    self.visit_yul_stmt_switch #_mut(switch);
                }
                yul::StmtKind::Leave => {}
                yul::StmtKind::Break => {}
                yul::StmtKind::Continue => {}
                yul::StmtKind::FunctionDef(function) => {
                    self.visit_yul_function #_mut(function);
                }
                yul::StmtKind::VarDecl(idents, expr) => {
                    for ident in idents {
                        self.visit_ident #_mut(ident);
                    }
                    if let Some(expr) = expr {
                        self.visit_yul_expr #_mut(expr);
                    }
                }
            }
        }

        fn visit_yul_block(&mut self, block: &'ast #mut yul::Block) {
            for stmt in block {
                self.visit_yul_stmt #_mut(stmt);
            }
        }

        fn visit_yul_stmt_switch(&mut self, switch: &'ast #mut yul::StmtSwitch) {
            let yul::StmtSwitch { selector, branches, default_case } = switch;
            self.visit_yul_expr #_mut(selector);
            for case in branches {
                self.visit_yul_stmt_case #_mut(case);
            }
            if let Some(case) = default_case {
                self.visit_yul_block #_mut(case);
            }
        }

        fn visit_yul_stmt_case(&mut self, case: &'ast #mut yul::StmtSwitchCase) {
            let yul::StmtSwitchCase { constant, body } = case;
            self.visit_lit #_mut(constant);
            self.visit_yul_block #_mut(body);
        }

        fn visit_yul_function(&mut self, function: &'ast #mut yul::Function) {
            let yul::Function { name, parameters, returns, body } = function;
            self.visit_ident #_mut(name);
            for ident in parameters {
                self.visit_ident #_mut(ident);
            }
            for ident in returns {
                self.visit_ident #_mut(ident);
            }
            self.visit_yul_block #_mut(body);
        }

        fn visit_yul_expr(&mut self, expr: &'ast #mut yul::Expr) {
            let yul::Expr { span, kind } = expr;
            self.visit_span #_mut(span);
            match kind {
                yul::ExprKind::Path(path) => {
                    self.visit_path #_mut(path);
                }
                yul::ExprKind::Call(call) => {
                    self.visit_yul_expr_call #_mut(call);
                }
                yul::ExprKind::Lit(lit) => {
                    self.visit_lit #_mut(lit);
                }
            }
        }

        fn visit_yul_expr_call(&mut self, call: &'ast #mut yul::ExprCall) {
            let yul::ExprCall { name, arguments } = call;
            self.visit_ident #_mut(name);
            for arg in arguments {
                self.visit_yul_expr #_mut(arg);
            }
        }

        fn visit_doc_comments(&mut self, doc_comments: &'ast #mut Vec<DocComment>) {
            for doc_comment in doc_comments {
                self.visit_doc_comment #_mut(doc_comment);
            }
        }

        fn visit_doc_comment(&mut self, doc_comment: &'ast #mut DocComment) {
            let DocComment { kind: _, span, symbol: _ } = doc_comment;
            self.visit_span #_mut(span);
        }

        fn visit_path(&mut self, path: &'ast #mut Path) {
            for ident in path.segments #_mut() {
                self.visit_ident #_mut(ident);
            }
        }

        fn visit_ident(&mut self, ident: &'ast #mut Ident) {
            let Ident { name: _, span } = ident;
            self.visit_span #_mut(span);
        }

        fn visit_span(&mut self, span: &'ast #mut Span) {
            // Nothing to do.
            let _ = span;
        }
    }
}