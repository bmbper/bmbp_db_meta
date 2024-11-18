use crate::meta::{
    RdbcColumnMeta, RdbcConstraintMeta, RdbcForeignKeyMeta, RdbcFuncMeta, RdbcIndexMeta,
    RdbcMetaResp, RdbcProducerMeta, RdbcSchemaMeta, RdbcTableMeta, RdbcTriggerMeta, RdbcUserMeta,
    RdbcViewMeta,
};

pub trait RdbcMetaQuery {
    fn schema_meta_all(&self) -> RdbcMetaResp<Vec<RdbcSchemaMeta>>;
    fn schema_meta(&self, schema: &str) -> RdbcMetaResp<Vec<RdbcSchemaMeta>>;
    fn user_schema_meta_all(&self, user: &str) -> RdbcMetaResp<Vec<RdbcSchemaMeta>>;
    fn user_schema_meta(&self, user: &str, schema: &str) -> RdbcMetaResp<Vec<RdbcSchemaMeta>>;
    fn table_meta_all(&self) -> RdbcMetaResp<Vec<RdbcTableMeta>>;
    fn table_meta(&self, table: &str) -> RdbcMetaResp<Vec<RdbcTableMeta>>;
    fn schema_table_all(&self, schema: &str) -> RdbcMetaResp<Vec<RdbcTableMeta>>;
    fn schema_table_meta(&self, schema: &str, table: &str) -> RdbcMetaResp<Vec<RdbcTableMeta>>;
    fn user_table_all(&self, user: &str) -> RdbcMetaResp<Vec<RdbcTableMeta>>;
    fn user_table_meta(&self, user: &str, table: &str) -> RdbcMetaResp<Vec<RdbcTableMeta>>;
    fn user_schema_table_all(&self, user: &str, schema: &str) -> RdbcMetaResp<Vec<RdbcTableMeta>>;
    fn user_schema_table_meta(
        &self,
        user: &str,
        schema: &str,
        table: &str,
    ) -> RdbcMetaResp<Vec<RdbcTableMeta>>;

    fn column_all(&self) -> RdbcMetaResp<Vec<RdbcColumnMeta>>;
    fn column_meta(&self, column_name: &str) -> RdbcMetaResp<Vec<RdbcColumnMeta>>;

    fn owner_column_all(&self, owner: &str) -> RdbcMetaResp<Vec<RdbcColumnMeta>>;
    fn owner_column_meta(&self, owner: &str, column: &str) -> RdbcMetaResp<Vec<RdbcColumnMeta>>;

    fn owner_table_column_all(&self, owner: &str, table: &str)
        -> RdbcMetaResp<Vec<RdbcColumnMeta>>;
    fn owner_table_column_meta(
        &self,
        owner: &str,
        table: &str,
        column: &str,
    ) -> RdbcMetaResp<Vec<RdbcColumnMeta>>;
    fn owner_schema_table_column_all(
        &self,
        owner: &str,
        schema: &str,
        table: &str,
    ) -> RdbcMetaResp<Vec<RdbcColumnMeta>>;
    fn owner_schema_table_column_meta(
        &self,
        owner: &str,
        schema: &str,
        table: &str,
        column: &str,
    ) -> RdbcMetaResp<Vec<RdbcColumnMeta>>;
    fn owner_schema_column_all(
        &self,
        owner: &str,
        schema: &str,
    ) -> RdbcMetaResp<Vec<RdbcColumnMeta>>;
    fn owner_schema_column_meta(
        &self,
        owner: &str,
        schema: &str,
        column: &str,
    ) -> RdbcMetaResp<Vec<RdbcColumnMeta>>;

    fn schema_column_all(&self, schema: &str) -> RdbcMetaResp<Vec<RdbcColumnMeta>>;
    fn schema_column_meta(&self, schema: &str, column: &str) -> RdbcMetaResp<Vec<RdbcColumnMeta>>;
    fn schema_table_column_all(
        &self,
        schema: &str,
        table: &str,
    ) -> RdbcMetaResp<Vec<RdbcColumnMeta>>;
    fn schema_table_column_meta(
        &self,
        schema: &str,
        table: &str,
        column: &str,
    ) -> RdbcMetaResp<Vec<RdbcColumnMeta>>;
    fn table_column_all(&self, table: &str) -> RdbcMetaResp<Vec<RdbcColumnMeta>>;
    fn table_column_meta(&self, table: &str, column: &str) -> RdbcMetaResp<Vec<RdbcColumnMeta>>;

    fn index_all(&self) -> RdbcMetaResp<Vec<RdbcIndexMeta>>;
    fn index_meta(&self, table: &str) -> RdbcMetaResp<Vec<RdbcIndexMeta>>;
    fn table_index_all(&self, table: &str) -> RdbcMetaResp<Vec<RdbcIndexMeta>>;
    fn table_index_meta(&self, table: &str, index: &str) -> RdbcMetaResp<Vec<RdbcIndexMeta>>;
    fn schema_table_index_all(&self, schema: &str, table: &str)
        -> RdbcMetaResp<Vec<RdbcIndexMeta>>;
    fn schema_table_index_meta(
        &self,
        schema: &str,
        table: &str,
        index: &str,
    ) -> RdbcMetaResp<Vec<RdbcIndexMeta>>;
    fn schema_index_all(&self, schema: &str) -> RdbcMetaResp<Vec<RdbcIndexMeta>>;
    fn schema_index_meta(&self, schema: &str, index: &str) -> RdbcMetaResp<Vec<RdbcIndexMeta>>;
    fn owner_index_all(&self, owner: &str) -> RdbcMetaResp<Vec<RdbcIndexMeta>>;
    fn owner_index_meta(&self, owner: &str, index: &str) -> RdbcMetaResp<Vec<RdbcIndexMeta>>;
    fn owner_table_index_all(&self, owner: &str, table: &str) -> RdbcMetaResp<Vec<RdbcIndexMeta>>;
    fn owner_table_index_meta(
        &self,
        owner: &str,
        table: &str,
        index: &str,
    ) -> RdbcMetaResp<Vec<RdbcIndexMeta>>;
    fn owner_schema_index_all(&self, owner: &str, schema: &str)
        -> RdbcMetaResp<Vec<RdbcIndexMeta>>;
    fn owner_schema_index_meta(
        &self,
        owner: &str,
        schema: &str,
        index: &str,
    ) -> RdbcMetaResp<Vec<RdbcIndexMeta>>;

    fn constraint_all(&self) -> RdbcMetaResp<Vec<RdbcConstraintMeta>>;
    fn constraint_meta(&self, table: &str) -> RdbcMetaResp<Vec<RdbcConstraintMeta>>;
    fn table_constraint_all(&self, table: &str) -> RdbcMetaResp<Vec<RdbcConstraintMeta>>;
    fn table_constraint_meta(
        &self,
        table: &str,
        constraint: &str,
    ) -> RdbcMetaResp<Vec<RdbcConstraintMeta>>;
    fn schema_constraint_all(&self, schema: &str) -> RdbcMetaResp<Vec<RdbcConstraintMeta>>;
    fn schema_constraint_meta(
        &self,
        schema: &str,
        constraint: &str,
    ) -> RdbcMetaResp<Vec<RdbcConstraintMeta>>;
    fn schema_table_constraint_all(
        &self,
        schema: &str,
        table: &str,
    ) -> RdbcMetaResp<Vec<RdbcConstraintMeta>>;
    fn schema_table_constraint_meta(
        &self,
        schema: &str,
        table: &str,
        constraint: &str,
    ) -> RdbcMetaResp<Vec<RdbcConstraintMeta>>;
    fn owner_constraint_all(&self, owner: &str) -> RdbcMetaResp<Vec<RdbcConstraintMeta>>;
    fn owner_constraint_meta(
        &self,
        owner: &str,
        constraint: &str,
    ) -> RdbcMetaResp<Vec<RdbcConstraintMeta>>;
    fn owner_table_constraint_all(
        &self,
        owner: &str,
        table: &str,
    ) -> RdbcMetaResp<Vec<RdbcConstraintMeta>>;
    fn owner_table_constraint_meta(
        &self,
        owner: &str,
        table: &str,
        constraint: &str,
    ) -> RdbcMetaResp<Vec<RdbcConstraintMeta>>;
    fn owner_schema_constraint_all(
        &self,
        owner: &str,
        schema: &str,
    ) -> RdbcMetaResp<Vec<RdbcConstraintMeta>>;
    fn owner_schema_constraint_meta(
        &self,
        owner: &str,
        schema: &str,
        constraint: &str,
    ) -> RdbcMetaResp<Vec<RdbcConstraintMeta>>;
    fn owner_schema_table_constraint_all(
        &self,
        owner: &str,
        schema: &str,
        table: &str,
    ) -> RdbcMetaResp<Vec<RdbcConstraintMeta>>;
    fn owner_schema_table_constraint_meta(
        &self,
        owner: &str,
        schema: &str,
        table: &str,
        constraint: &str,
    ) -> RdbcMetaResp<Vec<RdbcConstraintMeta>>;

    fn trigger_all(&self) -> RdbcMetaResp<Vec<RdbcTriggerMeta>>;
    fn trigger_meta(&self, trigger: &str) -> RdbcMetaResp<Vec<RdbcTriggerMeta>>;
    fn table_trigger_all(&self, trigger: &str) -> RdbcMetaResp<Vec<RdbcTriggerMeta>>;
    fn table_trigger_meta(&self, table: &str, trigger: &str) -> RdbcMetaResp<Vec<RdbcTriggerMeta>>;
    fn schema_trigger_all(&self, schema: &str) -> RdbcMetaResp<Vec<RdbcTriggerMeta>>;
    fn schema_trigger_meta(
        &self,
        schema: &str,
        trigger: &str,
    ) -> RdbcMetaResp<Vec<RdbcTriggerMeta>>;
    fn schema_table_trigger_all(
        &self,
        schema: &str,
        table: &str,
    ) -> RdbcMetaResp<Vec<RdbcTriggerMeta>>;
    fn schema_table_trigger_meta(
        &self,
        schema: &str,
        table: &str,
        trigger: &str,
    ) -> RdbcMetaResp<Vec<RdbcTriggerMeta>>;
    fn owner_trigger_all(&self, owner: &str) -> RdbcMetaResp<Vec<RdbcTriggerMeta>>;
    fn owner_trigger_meta(&self, owner: &str, trigger: &str) -> RdbcMetaResp<Vec<RdbcTriggerMeta>>;
    fn owner_table_trigger_all(
        &self,
        owner: &str,
        table: &str,
    ) -> RdbcMetaResp<Vec<RdbcTriggerMeta>>;
    fn owner_table_trigger_meta(
        &self,
        owner: &str,
        table: &str,
        trigger: &str,
    ) -> RdbcMetaResp<Vec<RdbcTriggerMeta>>;
    fn owner_schema_trigger_all(
        &self,
        owner: &str,
        schema: &str,
    ) -> RdbcMetaResp<Vec<RdbcTriggerMeta>>;
    fn owner_schema_trigger_meta(
        &self,
        owner: &str,
        schema: &str,
        trigger: &str,
    ) -> RdbcMetaResp<Vec<RdbcTriggerMeta>>;
    fn owner_schema_table_trigger_all(
        &self,
        owner: &str,
        schema: &str,
        table: &str,
    ) -> RdbcMetaResp<Vec<RdbcTriggerMeta>>;
    fn owner_schema_table_trigger_meta(
        &self,
        owner: &str,
        schema: &str,
        table: &str,
        trigger: &str,
    ) -> RdbcMetaResp<Vec<RdbcTriggerMeta>>;

    fn user_all(&self) -> RdbcMetaResp<Vec<RdbcUserMeta>>;
    fn user_meta(&self, user: &str) -> RdbcMetaResp<Vec<RdbcUserMeta>>;
    fn view_all(&self) -> RdbcMetaResp<Vec<RdbcViewMeta>>;
    fn view_meta(&self, view: &str) -> RdbcMetaResp<Vec<RdbcViewMeta>>;
    fn schema_view_all(&self, schema: &str) -> RdbcMetaResp<Vec<RdbcViewMeta>>;
    fn schema_view_meta(&self, schema: &str, view: &str) -> RdbcMetaResp<Vec<RdbcViewMeta>>;

    fn function_all(&self) -> RdbcMetaResp<Vec<RdbcFuncMeta>>;
    fn function_meta(&self, function: &str) -> RdbcMetaResp<Vec<RdbcFuncMeta>>;
    fn schema_function_all(&self, schema: &str) -> RdbcMetaResp<Vec<RdbcFuncMeta>>;
    fn schema_function_meta(&self, schema: &str, function: &str)
        -> RdbcMetaResp<Vec<RdbcFuncMeta>>;
    fn owner_function_all(&self, owner: &str) -> RdbcMetaResp<Vec<RdbcFuncMeta>>;
    fn owner_function_meta(&self, owner: &str, function: &str) -> RdbcMetaResp<Vec<RdbcFuncMeta>>;
    fn owner_schema_function_all(
        &self,
        owner: &str,
        schema: &str,
    ) -> RdbcMetaResp<Vec<RdbcFuncMeta>>;
    fn owner_schema_function_meta(
        &self,
        owner: &str,
        schema: &str,
        function: &str,
    ) -> RdbcMetaResp<Vec<RdbcFuncMeta>>;
    fn procedure_all(&self) -> RdbcMetaResp<Vec<RdbcProducerMeta>>;
    fn procedure_meta(&self, procedure: &str) -> RdbcMetaResp<Vec<RdbcProducerMeta>>;
    fn schema_procedure_all(&self, schema: &str) -> RdbcMetaResp<Vec<RdbcProducerMeta>>;
    fn schema_procedure_meta(
        &self,
        schema: &str,
        procedure: &str,
    ) -> RdbcMetaResp<Vec<RdbcProducerMeta>>;
    fn owner_procedure_all(&self, owner: &str) -> RdbcMetaResp<Vec<RdbcProducerMeta>>;
    fn owner_procedure_meta(
        &self,
        owner: &str,
        procedure: &str,
    ) -> RdbcMetaResp<Vec<RdbcProducerMeta>>;
    fn owner_schema_procedure_all(
        &self,
        owner: &str,
        schema: &str,
    ) -> RdbcMetaResp<Vec<RdbcProducerMeta>>;

    fn foreign_key_all(&self) -> RdbcMetaResp<Vec<RdbcForeignKeyMeta>>;
    fn foreign_key_meta(&self, foreign_key: &str) -> RdbcMetaResp<Vec<RdbcForeignKeyMeta>>;
    fn schema_foreign_key_all(&self, schema: &str) -> RdbcMetaResp<Vec<RdbcForeignKeyMeta>>;
    fn schema_foreign_key_meta(
        &self,
        schema: &str,
        foreign_key: &str,
    ) -> RdbcMetaResp<Vec<RdbcForeignKeyMeta>>;
    fn owner_foreign_key_all(&self, owner: &str) -> RdbcMetaResp<Vec<RdbcForeignKeyMeta>>;
    fn owner_foreign_key_meta(
        &self,
        owner: &str,
        foreign_key: &str,
    ) -> RdbcMetaResp<Vec<RdbcForeignKeyMeta>>;
    fn owner_schema_foreign_key_all(
        &self,
        owner: &str,
        schema: &str,
    ) -> RdbcMetaResp<Vec<RdbcForeignKeyMeta>>;
    fn owner_schema_foreign_key_meta(
        &self,
        owner: &str,
        schema: &str,
        foreign_key: &str,
    ) -> RdbcMetaResp<Vec<RdbcForeignKeyMeta>>;
    fn owner_table_foreign_key_all(
        &self,
        owner: &str,
        table: &str,
    ) -> RdbcMetaResp<Vec<RdbcForeignKeyMeta>>;
    fn owner_table_foreign_key_meta(
        &self,
        owner: &str,
        table: &str,
        foreign_key: &str,
    ) -> RdbcMetaResp<Vec<RdbcForeignKeyMeta>>;
    fn owner_schema_table_foreign_key_all(
        &self,
        owner: &str,
        schema: &str,
        table: &str,
    ) -> RdbcMetaResp<Vec<RdbcForeignKeyMeta>>;
    fn owner_schema_table_foreign_key_meta(
        &self,
        owner: &str,
        schema: &str,
        table: &str,
        foreign_key: &str,
    ) -> RdbcMetaResp<Vec<RdbcForeignKeyMeta>>;
    fn table_foreign_key_all(&self, table: &str) -> RdbcMetaResp<Vec<RdbcForeignKeyMeta>>;
    fn table_foreign_key_meta(
        &self,
        table: &str,
        foreign_key: &str,
    ) -> RdbcMetaResp<Vec<RdbcForeignKeyMeta>>;
    fn schema_table_foreign_key_all(
        &self,
        schema: &str,
        table: &str,
    ) -> RdbcMetaResp<Vec<RdbcForeignKeyMeta>>;
    fn schema_table_foreign_key_meta(
        &self,
        schema: &str,
        table: &str,
        foreign_key: &str,
    ) -> RdbcMetaResp<Vec<RdbcForeignKeyMeta>>;
}

pub trait RdbcMetaBuilder {
    fn create_user(&self, user: &RdbcUserMeta) -> Vec<String>;
    fn alter_user(&self, user: &RdbcUserMeta) -> Vec<String>;
    fn drop_user(&self, user: &RdbcUserMeta) -> Vec<String>;
    fn create_schema(&self, schema: &RdbcSchemaMeta) -> Vec<String>;
    fn alter_schema(&self, schema: &RdbcSchemaMeta) -> Vec<String>;
    fn drop_schema(&self, schema: &RdbcSchemaMeta) -> Vec<String>;
    fn create_table(&self, table: &RdbcTableMeta) -> Vec<String>;
    fn alter_table(&self, table: &RdbcTableMeta) -> Vec<String>;
    fn drop_table(&self, table: &RdbcTableMeta) -> Vec<String>;
    fn create_view(&self, view: &RdbcViewMeta) -> Vec<String>;
    fn alter_view(&self, view: &RdbcViewMeta) -> Vec<String>;
    fn drop_view(&self, view: &RdbcViewMeta) -> Vec<String>;
    fn create_function(&self, func: &RdbcFuncMeta) -> Vec<String>;
    fn alter_function(&self, func: &RdbcFuncMeta) -> Vec<String>;
    fn drop_function(&self, func: &RdbcFuncMeta) -> Vec<String>;
    fn create_procedure(&self, producer: &RdbcProducerMeta) -> Vec<String>;
    fn alter_procedure(&self, producer: &RdbcProducerMeta) -> Vec<String>;
    fn drop_procedure(&self, producer: &RdbcProducerMeta) -> Vec<String>;
    fn create_trigger(&self, trigger: &RdbcTriggerMeta) -> Vec<String>;
    fn alter_trigger(&self, trigger: &RdbcTriggerMeta) -> Vec<String>;
    fn drop_trigger(&self, trigger: &RdbcTriggerMeta) -> Vec<String>;
    fn create_constraint(&self, constraint: &RdbcConstraintMeta) -> Vec<String>;
    fn alter_constraint(&self, constraint: &RdbcConstraintMeta) -> Vec<String>;
    fn drop_constraint(&self, constraint: &RdbcConstraintMeta) -> Vec<String>;
    fn create_index(&self, index: &RdbcIndexMeta) -> Vec<String>;
    fn alter_index(&self, index: &RdbcIndexMeta) -> Vec<String>;
    fn drop_index(&self, index: &RdbcIndexMeta) -> Vec<String>;
    fn create_foreign_key(&self, foreign_key: &RdbcForeignKeyMeta) -> Vec<String>;
    fn alter_foreign_key(&self, foreign_key: &RdbcForeignKeyMeta) -> Vec<String>;
    fn drop_foreign_key(&self, foreign_key: &RdbcForeignKeyMeta) -> Vec<String>;
    fn create_column(&self, column: &RdbcColumnMeta) -> Vec<String>;
    fn alter_column(&self, column: &RdbcColumnMeta) -> Vec<String>;
    fn drop_column(&self, column: &RdbcColumnMeta) -> Vec<String>;
    fn migrate(&self, table: &RdbcTableMeta) -> Vec<String>;
}

pub trait RdbcMetaExecutor {
    fn create_user(&self, user: &RdbcUserMeta) -> RdbcMetaResp<usize>;
    fn alter_user(&self, user: &RdbcUserMeta) -> RdbcMetaResp<usize>;
    fn drop_user(&self, user: &RdbcUserMeta) -> RdbcMetaResp<usize>;
    fn create_schema(&self, schema: &RdbcSchemaMeta) -> RdbcMetaResp<usize>;
    fn alter_schema(&self, schema: &RdbcSchemaMeta) -> RdbcMetaResp<usize>;
    fn drop_schema(&self, schema: &RdbcSchemaMeta) -> RdbcMetaResp<usize>;
    fn create_table(&self, table: &RdbcTableMeta) -> RdbcMetaResp<usize>;
    fn alter_table(&self, table: &RdbcTableMeta) -> RdbcMetaResp<usize>;
    fn drop_table(&self, table: &RdbcTableMeta) -> RdbcMetaResp<usize>;
    fn create_view(&self, view: &RdbcViewMeta) -> RdbcMetaResp<usize>;
    fn alter_view(&self, view: &RdbcViewMeta) -> RdbcMetaResp<usize>;
    fn drop_view(&self, view: &RdbcViewMeta) -> RdbcMetaResp<usize>;
    fn create_function(&self, func: &RdbcFuncMeta) -> RdbcMetaResp<usize>;
    fn alter_function(&self, func: &RdbcFuncMeta) -> RdbcMetaResp<usize>;
    fn drop_function(&self, func: &RdbcFuncMeta) -> RdbcMetaResp<usize>;
    fn create_procedure(&self, producer: &RdbcProducerMeta) -> RdbcMetaResp<usize>;
    fn alter_procedure(&self, producer: &RdbcProducerMeta) -> RdbcMetaResp<usize>;
    fn drop_procedure(&self, producer: &RdbcProducerMeta) -> RdbcMetaResp<usize>;
    fn create_trigger(&self, trigger: &RdbcTriggerMeta) -> RdbcMetaResp<usize>;
    fn alter_trigger(&self, trigger: &RdbcTriggerMeta) -> RdbcMetaResp<usize>;
    fn drop_trigger(&self, trigger: &RdbcTriggerMeta) -> RdbcMetaResp<usize>;
    fn create_constraint(&self, constraint: &RdbcConstraintMeta) -> RdbcMetaResp<usize>;
    fn alter_constraint(&self, constraint: &RdbcConstraintMeta) -> RdbcMetaResp<usize>;
    fn drop_constraint(&self, constraint: &RdbcConstraintMeta) -> RdbcMetaResp<usize>;
    fn create_index(&self, index: &RdbcIndexMeta) -> RdbcMetaResp<usize>;
    fn alter_index(&self, index: &RdbcIndexMeta) -> RdbcMetaResp<usize>;
    fn drop_index(&self, index: &RdbcIndexMeta) -> RdbcMetaResp<usize>;
    fn create_foreign_key(&self, foreign_key: &RdbcForeignKeyMeta) -> RdbcMetaResp<usize>;
    fn alter_foreign_key(&self, foreign_key: &RdbcForeignKeyMeta) -> RdbcMetaResp<usize>;
    fn drop_foreign_key(&self, foreign_key: &RdbcForeignKeyMeta) -> RdbcMetaResp<usize>;
    fn create_column(&self, column: &RdbcColumnMeta) -> RdbcMetaResp<usize>;
    fn alter_column(&self, column: &RdbcColumnMeta) -> RdbcMetaResp<usize>;
    fn drop_column(&self, column: &RdbcColumnMeta) -> RdbcMetaResp<usize>;
    fn migrate(&self, table: &RdbcTableMeta) -> RdbcMetaResp<usize>;
}
