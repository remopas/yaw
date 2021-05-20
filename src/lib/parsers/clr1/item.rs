
use crate::lib::parsers::common::{ Symbol, Rule, Item };

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct LR1Item<'r>{
    rule: &'r Rule,
    index: usize,
    look_ahead: Symbol
}

impl<'r> LR1Item<'r> {

    pub fn new(rule: &'r Rule, index: usize, look_ahead: Symbol) -> Self {
        return Self {rule, index, look_ahead}
    }

    pub fn get_following_active(&self) -> &Symbol {
        if self.index + 1 < self.rule.get_symbols().len() {
            return &self.rule.get_symbols()[self.index + 1];
        } else {
            return self.get_look_ahead();
        }
    }

    pub fn get_look_ahead(&self) -> &Symbol {
        return &self.look_ahead;
    }
    
}

impl<'r> Item for LR1Item<'r> {

    fn get_index(&self) -> usize {
        return self.index;
    }

    fn get_rule(&self) -> &Rule {
        return self.rule;
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

}

