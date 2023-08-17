pub struct FuncType<'a> {
  __something: &'a str,
}
pub struct Func<'a> {
  __something: &'a str,
}
pub struct Table<'a> {
  __something: &'a str,
}
pub struct Memory<'a> {
  __something: &'a str,
}
pub struct Global<'a> {
  __something: &'a str,
}
pub struct ElemSegment<'a> {
  __something: &'a str,
}
pub struct DataSegment<'a> {
  __something: &'a str,
}
pub struct Export<'a> {
  __something: &'a str,
}
pub struct StartFunction<'a> {
  __something: &'a str,
}

// https://webassembly.github.io/spec/core/syntax/modules.html
pub struct Module<'a> {
  pub types: Vec<FuncType<'a>>,
  pub funcs: Vec<Func<'a>>,
  pub tables: Vec<Table<'a>>,
  pub mems: Vec<Memory<'a>>,
  pub globals: Vec<Global<'a>>,
  pub elems: Vec<ElemSegment<'a>>,
  pub datas: Vec<DataSegment<'a>>,
  pub start: usize,
  pub entrypoint: StartFunction<'a>,
  pub exports: Vec<Export<'a>>,
}

// https://webassembly.github.io/spec/core/syntax/modules.html#indices
pub type TypeIdx = u32;
pub type FuncIdx = u32;
pub type TableIdx = u32;
pub type MemIdx = u32;
pub type GlobalIdx = u32;
pub type LocalIdx = u32;
pub type LabelIdx = u32;
