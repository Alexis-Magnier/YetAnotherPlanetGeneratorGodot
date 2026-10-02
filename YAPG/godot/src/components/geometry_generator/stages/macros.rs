#[macro_export]
macro_rules! delegate_inner {
    // ------------------------------------------------------------
    // Finished parsing: emit the actual Godot impl
    // ------------------------------------------------------------
    (@finish $struct_name:ident { $($body:tt)* }) => {
        ::paste::paste! {
            #[godot_api]
            impl $struct_name {
                $($body)*
            }
        }
    };

    // ------------------------------------------------------------
    // End of input -> finish
    // ------------------------------------------------------------
    (
        @parse
        $struct_name:ident
        { $($body:tt)* }
    ) => {
        delegate_inner!(
            @finish
            $struct_name
            {
                $($body)*
            }
        );
    };

    // ------------------------------------------------------------
    // Custom getter + setter
    // ------------------------------------------------------------
    (
        @parse
        $struct_name:ident
        { $($body:tt)* }

        $field:ident : $type:ty
        => {
            get => $get:expr,
            set => |$val:ident| $set:expr
        }

        , $($rest:tt)*
    ) => {
        delegate_inner!(
            @parse
            $struct_name
            {
                $($body)*

                #[func]
                pub fn [<set_ $field>](&mut self, $val: $type) {
                    $set
                }

                #[func]
                pub fn [<get_ $field>](&self) -> $type {
                    $get
                }
            }
            $($rest)*
        );
    };

        // Custom getter + setter
    (
        @parse
        $struct_name:ident
        { $($body:tt)* }

        $field:ident : $type:ty
        => {
            get => |$this:ident| $get:expr,
            set => |$set_this:ident, $val:ident| $set:expr
        }

        , $($rest:tt)*
    ) => {
        delegate_inner!(
            @parse
            $struct_name
            {
                $($body)*

                #[func]
                pub fn [<set_ $field>](&mut self, $val: $type) {
                    let $set_this = self;
                    $set
                }

                #[func]
                pub fn [<get_ $field>](&self) -> $type {
                    let $this = self;
                    $get
                }
            }
            $($rest)*
        );
    };

    // Custom getter + setter, last item
    (
        @parse
        $struct_name:ident
        { $($body:tt)* }

        $field:ident : $type:ty
        => {
            get => |$this:ident| $get:expr,
            set => |$set_this:ident, $val:ident| $set:expr
        }

        ,
    ) => {
        delegate_inner!(
            @parse
            $struct_name
            {
                $($body)*

                #[func]
                pub fn [<set_ $field>](&mut self, $val: $type) {
                    let $set_this = self;
                    $set
                }

                #[func]
                pub fn [<get_ $field>](&self) -> $type {
                    let $this = self;
                    $get
                }
            }
        );
};

    // ------------------------------------------------------------
    // Default getter + setter
    // ------------------------------------------------------------
    (
        @parse
        $struct_name:ident
        { $($body:tt)* }

        $field:ident : $type:ty
        , $($rest:tt)*
    ) => {
        delegate_inner!(
            @parse
            $struct_name
            {
                $($body)*

                #[func]
                pub fn [<set_ $field>](&mut self, val: $type) {
                    self.inner.$field = val;
                }

                #[func]
                pub fn [<get_ $field>](&self) -> $type {
                    self.inner.$field.clone()
                }
            }
            $($rest)*
        );
    };

    // Default property, last item
    (
        @parse
        $struct_name:ident
        { $($body:tt)* }

        $field:ident : $type:ty
        ,
    ) => {
        delegate_inner!(
            @parse
            $struct_name
            {
                $($body)*

                #[func]
                pub fn [<set_ $field>](&mut self, val: $type) {
                    self.inner.$field = val;
                }

                #[func]
                pub fn [<get_ $field>](&self) -> $type {
                    self.inner.$field.clone()
                }
            }
        );
    };

    // ------------------------------------------------------------
    // Entry point
    // ------------------------------------------------------------
    (
        $struct_name:ident {
            $($fields:tt)*
        }
    ) => {
        delegate_inner!(
            @parse
            $struct_name
            {}
            $($fields)*
        );
    };
}