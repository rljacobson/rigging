use codemap::Span;

use crate::abstractions::BigInteger;

use crate::parser::location::{Located, SourceLocation};

// Type alias for text
type Text = String;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeData {
  Object(Vec<(String, AttributeData)>),
  List(Vec<AttributeData>),
  Num(BigInteger),
  String(String),
  Bool(bool),
}

pub type LocatedAttributeData = Located<AttributeData>;

/// External binding information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalBindings {
  pub is_pure: bool,
  pub bindings: Vec<(String, String)>,
}


// Aliases for identifier and infix identifier
type Identifier = Text;
type InfixIdentifier = Text;

/// Enum for kind
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
  /// Base kind of types
  Type,
  /// kind of natural number size expressions
  Integer,
  /// kind of vector order specifications
  Order,
  /// kind of constraints
  Bool,
}

/// Kind with location
pub type LocatedKind = Located<Kind>;

/// Identifiers with kind, ticked to differentiate from program variables
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindIdentifier(pub Identifier);

/// Kind ID with location
pub type LocatedKindIdentifier = Located<KindIdentifier>;

/// Enum for identifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentifierType {
  Regular(Identifier),
  Operator(Identifier), // remove infix status
}

/// Id with location
pub type LocatedIdentifier = Located<IdentifierType>;

/// Enum for infix token
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfixToken<T> {
  Primary(T),
  Operator(LocatedIdentifier),
  Prefix(LocatedIdentifier),
}

/// Represents various types of literals
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
  /// Unit literal
  Unit,
  /// Zero literal
  Zero,
  /// One literal
  One,
  /// Boolean true literal
  True,
  /// Boolean false literal
  False,
  /// Natural number constant
  Number(BigInteger),
  /// Bit vector constant in hexadecimal format
  Hexadecimal(String),
  /// Bit vector constant in binary format
  Binary(String),
  /// Undefined value
  Undefined,
  /// String constant
  String(String),
  /// Real number constant
  Real(String),
}

/// Literal with location
pub type LocatedLiteral = Located<Literal>;

/// Represents various types of abstract types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractType {
  /// Identifier type
  Identifier(LocatedIdentifier),
  /// Ticked variable type
  Variable(LocatedKindIdentifier),
  /// Literal type
  Literal(LocatedLiteral),
  /// Set type with natural number constants
  NumberSet(Vec<BigInteger>),
  /// Set type with two abstract types
  In(Box<LocatedAbstractType>, Box<LocatedAbstractType>),
  /// Product type with two abstract types
  Times(Box<LocatedAbstractType>, Box<LocatedAbstractType>),
  /// Sum type with two abstract types
  Sum(Box<LocatedAbstractType>, Box<LocatedAbstractType>),
  /// Subtraction type with two abstract types
  Minus(Box<LocatedAbstractType>, Box<LocatedAbstractType>),
  /// Exponential type with an abstract type
  Exponential(Box<LocatedAbstractType>),
  /// Negative type with an abstract type
  Negative(Box<LocatedAbstractType>),
  /// Infix type with a list of infix tokens and their positions
  Infix(Vec<(InfixToken<LocatedAbstractType>, Span)>),
  /// Increasing type
  Increasing,
  /// Decreasing type
  Decreasing,
  /// Effect set type with a list of identifiers
  EffectSet(Vec<LocatedIdentifier>),
  /// Function type, the last thing is an effect
  Function{
    lhs: Box<LocatedAbstractType>,
    rhs: Box<LocatedAbstractType>,
    effect: Box<LocatedAbstractType>
  },
  /// Mapping type between two things, the third thing is an effect
  Bidirectional{
    lhs: Box<LocatedAbstractType>,
    rhs: Box<LocatedAbstractType>,
    effect: Box<LocatedAbstractType>
  },
  /// Wildcard type
  Wildcard,
  Tuple(Vec<LocatedAbstractType>),
  TypeConstructorApplication(LocatedIdentifier, Vec<LocatedAbstractType>),
  /// Conditional of the form if first then second else third
  If{
    condition: Box<LocatedAbstractType>,
    then     : Box<LocatedAbstractType>,
    elsewise : Box<LocatedAbstractType>
  },
  Existential(Vec<LocatedKindIdentifier>, Box<LocatedAbstractType>, Box<LocatedAbstractType>),
  Parenthesized(Box<LocatedAbstractType>),
}

/// AbstractType with location
pub type LocatedAbstractType = Located<AbstractType>;


/// Kind-annotated variable with optional string, list of kind identifiers, and optional kind
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindedIdentifier {
  identifiers: Vec<LocatedKindIdentifier>,
  annotation : Option<String>,
  kind       : Option<LocatedKind>,
}

/// KindedIdentifier with location
pub type LocatedKindedIdentifier = Located<KindedIdentifier>;

/// Represents items in a quantifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantifierItem {
  /// An optionally kinded identifier
  KindedIdentifier(LocatedKindedIdentifier),
  /// A constraint for this type
  Constraint(LocatedAbstractType),
}

/// QuantifierItem with location
pub type LocatedQuantifierItem = Located<QuantifierItem>;

/// Represents type quantifiers and constraints
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeQuantifier {
  /// Type quantifiers with a list of quantifier items
  TypeQuantifiers(Vec<LocatedQuantifierItem>),
  /// Sugar, omitting quantifier and constraints
  NoForAll,
}

/// TypeQuantifier with location
pub type LocatedTypeQuantifier = Located<TypeQuantifier>;

/// Represents a type scheme
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeScheme {
  /// Type quantifier
  pub quantifier: LocatedTypeQuantifier,
  /// Abstract type
  pub abstract_type: LocatedAbstractType,
}

/// TypeScheme with location
pub type LocatedTypeScheme = Located<TypeScheme>;

/// Represents various types of patterns
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
  /// Literal constant pattern
  Literal(LocatedLiteral),
  /// Wildcard pattern (always matches)
  Wildcard,
  Typed(Box<LocatedAbstractType>, Box<LocatedPattern>),
  Identifier(LocatedIdentifier),
  /// Bind pattern to type variable
  Variable(Box<LocatedPattern>, Box<LocatedAbstractType>),
  /// Union constructor pattern
  Constructor(LocatedIdentifier, Vec<LocatedPattern>),
  /// Vector pattern
  Vector(Vec<LocatedPattern>),
  /// Concatenated vector pattern
  VectorConcat(Vec<LocatedPattern>),
  /// Vector subrange pattern
  VectorSubrange(LocatedIdentifier, BigInteger, BigInteger),
  Tuple(Vec<LocatedPattern>),
  List(Vec<LocatedPattern>),
  Cons(Box<LocatedPattern>, Box<LocatedPattern>),
  /// String append pattern (x ^^ y)
  StringAppend(Vec<LocatedPattern>),
  Struct(Vec<LocatedFieldPattern>),
  Attribute(String, Option<LocatedAttributeData>, Box<LocatedPattern>),
}

/// Pattern with location
pub type LocatedPattern = Located<Pattern>;

/// Represents various types of field patterns
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldPattern {
  /// Field pattern
  Field(LocatedIdentifier, Box<LocatedPattern>),
  /// Wildcard field pattern
  Wildcard,
}

/// FieldPattern with location
pub type LocatedFieldPattern = Located<FieldPattern>;

/// Represents loop types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopType {
  While,
  Until,
}

/// If location structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfLocation {
  pub if_loc: SourceLocation,
  pub then_loc: SourceLocation,
  pub else_loc: Option<SourceLocation>,
}

/// Represents optional termination measure for a loop
pub type Measure = Option<LocatedExpression>;

/// Measure with location
pub type LocatedMeasure = Located<Measure>;

/// Represents various types of expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
  /// Block expression
  Block(Vec<LocatedExpression>),
  /// Identifier expression
  Identifier(LocatedIdentifier),
  /// Reference to identifier
  Reference(LocatedIdentifier),
  /// Dereference expression
  Dereference(Box<LocatedExpression>),
  /// Literal constant expression
  Literal(LocatedLiteral),
  /// Cast expression
  Typed(Box<LocatedAbstractType>, Box<LocatedExpression>),
  /// Function application expression
  Application(LocatedIdentifier, Vec<LocatedExpression>),
  /// Infix function application expression
  InfixApplication(Box<LocatedExpression>, LocatedIdentifier, Box<LocatedExpression>),
  /// Infix expression with a list of infix tokens and their positions
  Infix(Vec<(InfixToken<LocatedExpression>, Span)>),
  /// Tuple expression
  Tuple(Vec<LocatedExpression>),
  /// Conditional expression
  If {
    condition: Box<LocatedExpression>,
    then_expr: Box<LocatedExpression>,
    else_expr: Box<LocatedExpression>,
    if_location: IfLocation,
  },
  /// Loop expression
  Loop(LoopType, LocatedMeasure, Box<LocatedExpression>, Box<LocatedExpression>),
  /// For loop expression
  For {
    identifier: LocatedIdentifier,
    start: Box<LocatedExpression>,
    end: Box<LocatedExpression>,
    step: Box<LocatedExpression>,
    typ: Box<LocatedAbstractType>,
    body: Box<LocatedExpression>,
  },
  /// Vector expression
  Vector(Vec<LocatedExpression>),
  /// Vector access expression
  VectorAccess(Box<LocatedExpression>, Box<LocatedExpression>),
  /// Subvector extraction expression
  VectorSubrange(Box<LocatedExpression>, Box<LocatedExpression>, Box<LocatedExpression>),
  /// Vector functional update expression
  VectorUpdate(Box<LocatedExpression>, Box<LocatedExpression>, Box<LocatedExpression>),
  /// Vector subrange update expression
  VectorUpdateSubrange(Box<LocatedExpression>, Box<LocatedExpression>, Box<LocatedExpression>, Box<LocatedExpression>),
  /// Vector concatenation expression
  VectorAppend(Box<LocatedExpression>, Box<LocatedExpression>),
  /// List expression
  List(Vec<LocatedExpression>),
  /// Cons expression
  Cons(Box<LocatedExpression>, Box<LocatedExpression>),
  /// Struct expression
  Struct(Vec<LocatedExpression>),
  /// Functional update of struct expression
  StructUpdate(Box<LocatedExpression>, Vec<LocatedExpression>),
  /// Field projection from struct expression
  Field(Box<LocatedExpression>, LocatedIdentifier),
  /// Pattern matching expression
  Match(Box<LocatedExpression>, Vec<LocatedPatternExpression>),
  /// Let expression
  Let(LocatedLetBinding, Box<LocatedExpression>),
  /// Imperative assignment expression
  Assign(Box<LocatedExpression>, Box<LocatedExpression>),
  /// Sizeof expression
  Sizeof(Box<LocatedAbstractType>),
  /// Constraint expression
  Constraint(Box<LocatedAbstractType>),
  /// Exit expression
  Exit(Box<LocatedExpression>),
  /// Throw expression
  Throw(Box<LocatedExpression>),
  /// Try expression
  Try(Box<LocatedExpression>, Vec<LocatedPatternExpression>),
  /// Return expression
  Return(Box<LocatedExpression>),
  /// Assert expression
  Assert(Box<LocatedExpression>, Box<LocatedExpression>),
  /// Variable expression
  Variable(Box<LocatedExpression>, Box<LocatedExpression>, Box<LocatedExpression>),
  /// Attribute expression
  Attribute(String, Option<LocatedAttributeData>, Box<LocatedExpression>),
  /// Internal plet expression
  InternalPlet(Box<LocatedPattern>, Box<LocatedExpression>, Box<LocatedExpression>),
  /// Internal return expression
  InternalReturn(Box<LocatedExpression>),
  /// Internal assume expression
  InternalAssume(Box<LocatedAbstractType>, Box<LocatedExpression>),
}

/// Expression with location
pub type LocatedExpression = Located<Expression>;

/// Represents optional default value for indexed vectors
pub type OptionalDefault = Option<LocatedExpression>;

/// Optional default value with location
pub type LocatedOptionalDefault = Located<OptionalDefault>;

/// Represents pattern match
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternExpression {
  Pattern(Box<LocatedPattern>, Box<LocatedExpression>),
  PatternWhen(Box<LocatedPattern>, Box<LocatedExpression>, Box<LocatedExpression>),
}

/// PatternExpression with location
pub type LocatedPatternExpression = Located<PatternExpression>;

/// Represents let binding
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LetBinding {
  ValueBinding(Box<LocatedPattern>, Box<LocatedExpression>),
}

/// LetBinding with location
pub type LocatedLetBinding = Located<LetBinding>;

/// Represents optional type annotation for functions
pub type TypeAnnotationOption = Option<(LocatedTypeQuantifier, Box<LocatedAbstractType>)>;

/// TypeAnnotationOption with location
pub type LocatedTypeAnnotationOption = Located<TypeAnnotationOption>;

/// Represents optional type scheme
pub type TypeSchemeOption = Option<LocatedTypeScheme>;

/// TypeSchemeOption with location
pub type LocatedTypeSchemeOption = Located<TypeSchemeOption>;

/// Represents optional effect annotation for functions
pub type EffectOption = Option<LocatedAbstractType>;

/// EffectOption with location
pub type LocatedEffectOption = Located<EffectOption>;

/// Represents optional recursive annotation for functions
pub type RecursiveMeasureOption = Option<(Box<LocatedPattern>, Box<LocatedExpression>)>;

/// RecursiveOption with location
pub type LocatedRecursiveOption = Located<RecursiveMeasureOption>;

/// Represents function clause
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionClause {
  Private(Box<LocatedFunctionClause>),
  Attribute(String, Option<LocatedAttributeData>, Box<LocatedFunctionClause>),
  Documentation(String, Box<LocatedFunctionClause>),
  Clause(LocatedIdentifier, Box<LocatedPatternExpression>),
}

/// FunctionClause with location
pub type LocatedFunctionClause = Located<FunctionClause>;

/// Represents type union constructors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeUnion {
  Private(Box<LocatedTypeUnion>),
  Attribute(String, Option<LocatedAttributeData>, Box<LocatedTypeUnion>),
  Documentation(String, Box<LocatedTypeUnion>),
  TypeIdentifier(Box<LocatedAbstractType>, LocatedIdentifier),
  AnonymousRecord(Vec<(LocatedAbstractType, LocatedIdentifier)>, LocatedIdentifier),
}

/// TypeUnion with location
pub type LocatedTypeUnion = Located<TypeUnion>;

/// Represents instantiation substitution
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstantiationSubstitution {
  TypeSubstitution(LocatedKindIdentifier, Box<LocatedAbstractType>),
  IdentifierSubstitution(LocatedIdentifier, LocatedIdentifier),
}

/// InstantiationSubstitution with location
pub type LocatedInstantiationSubstitution = Located<InstantiationSubstitution>;

/// Represents index specification for bitfields in register types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexRange {
  /// Single index
  Single(Box<LocatedAbstractType>),
  /// Index range
  Range(Box<LocatedAbstractType>, Box<LocatedAbstractType>),
  /// Concatenation of index ranges
  Concat(Box<LocatedIndexRange>, Box<LocatedIndexRange>),
}

/// IndexRange with location
pub type LocatedIndexRange = Located<IndexRange>;

/// Represents default kinding or typing assumption and default order for literal vectors and vector shorthands
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefaultTypingSpec {
  Order(LocatedKind, Box<LocatedAbstractType>),
}

/// DefaultTypingSpec with location
pub type LocatedDefaultTypingSpec = Located<DefaultTypingSpec>;

/// Represents mapping pattern
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MappingPattern {
  Literal(LocatedLiteral),
  Identifier(LocatedIdentifier),
  Application(LocatedIdentifier, Vec<LocatedMappingPattern>),
  Vector(Vec<LocatedMappingPattern>),
  VectorConcat(Vec<LocatedMappingPattern>),
  VectorSubrange(LocatedIdentifier, BigInteger, BigInteger),
  Tuple(Vec<LocatedMappingPattern>),
  List(Vec<LocatedMappingPattern>),
  Cons(Box<LocatedMappingPattern>, Box<LocatedMappingPattern>),
  StringAppend(Vec<LocatedMappingPattern>),
  Typed(Box<LocatedMappingPattern>, Box<LocatedAbstractType>),
  As(Box<LocatedMappingPattern>, LocatedIdentifier),
  Struct(Vec<(LocatedIdentifier, LocatedMappingPattern)>),
}

/// MappingPattern with location
pub type LocatedMappingPattern = Located<MappingPattern>;

/// Represents mapping pattern expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MappingPatternExpression {
  Pattern(LocatedMappingPattern),
  PatternWhen(LocatedMappingPattern, Box<LocatedExpression>),
}

/// MappingPatternExpression with location
pub type LocatedMappingPatternExpression = Located<MappingPatternExpression>;

/// Represents mapping clause (bidirectional pattern-match)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MappingClause {
  Attribute(String, Option<LocatedAttributeData>, Box<LocatedMappingClause>),
  Documentation(String, Box<LocatedMappingClause>),
  Bidirectional(LocatedMappingPatternExpression, LocatedMappingPatternExpression),
  ForwardsDeprecated(LocatedMappingPatternExpression, Box<LocatedExpression>),
  Forwards(LocatedPatternExpression),
  Backwards(LocatedPatternExpression),
}

/// MappingClause with location
pub type LocatedMappingClause = Located<MappingClause>;

/// Represents mapping definition (bidirectional pattern-match function)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MappingDefinition {
  Mapping(LocatedIdentifier, LocatedTypeSchemeOption, Vec<LocatedMappingClause>),
}

/// MappingDefinition with location
pub type LocatedMappingDefinition = Located<MappingDefinition>;

/// Represents outcome declaration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutcomeSpec {
  Outcome(LocatedIdentifier, Box<LocatedTypeScheme>, Vec<LocatedKindIdentifier>),
}

/// OutcomeSpec with location
pub type LocatedOutcomeSpec = Located<OutcomeSpec>;

/// Represents function definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionDefinition {
  Function(
    LocatedRecursiveOption,
    LocatedTypeAnnotationOption,
    LocatedEffectOption,
    Vec<LocatedFunctionClause>,
  ),
}

/// FunctionDefinition with location
pub type LocatedFunctionDefinition = Located<FunctionDefinition>;

/// Represents type definition body
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDefinition {
  Abbreviation(LocatedIdentifier, LocatedTypeQuantifier, LocatedKind, Box<LocatedAbstractType>),
  Record(LocatedIdentifier, LocatedTypeQuantifier, Vec<(LocatedAbstractType, LocatedIdentifier)>),
  Variant(LocatedIdentifier, LocatedTypeQuantifier, Vec<LocatedTypeUnion>),
  Enum(
    LocatedIdentifier,
    Vec<(LocatedIdentifier, Box<LocatedAbstractType>)>,
    Vec<(LocatedIdentifier, Option<Box<LocatedExpression>>)>,
  ),
  Abstract(LocatedIdentifier, LocatedKind),
  Bitfield(
    LocatedIdentifier,
    Box<LocatedAbstractType>,
    Vec<(LocatedIdentifier, LocatedIndexRange)>,
  ),
}

/// TypeDefinition with location
pub type LocatedTypeDefinition = Located<TypeDefinition>;

/// Represents value type specification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueSpecification {
  ValueSpec(Box<LocatedTypeScheme>, LocatedIdentifier, Option<ExternalBindings>),
}

/// ValueSpecification with location
pub type LocatedValueSpecification = Located<ValueSpecification>;

/// Represents register declarations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclarationSpecification {
  Register(Box<LocatedAbstractType>, LocatedIdentifier, Option<Box<LocatedExpression>>),
}

/// DeclarationSpecification with location
pub type LocatedDeclarationSpecification = Located<DeclarationSpecification>;

/// Represents scattered function and type union definitions that can be spread across a file
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScatteredDefinition {
  Function(
    LocatedRecursiveOption,
    LocatedTypeAnnotationOption,
    LocatedEffectOption,
    LocatedIdentifier,
  ),
  FunctionClause(LocatedFunctionClause),
  Enumeration(LocatedIdentifier),
  EnumerationMember(LocatedIdentifier, LocatedIdentifier),
  Variant(LocatedIdentifier, LocatedTypeQuantifier),
  UnionClause(LocatedIdentifier, LocatedTypeUnion),
  Mapping(LocatedIdentifier, LocatedTypeAnnotationOption),
  MapClause(LocatedIdentifier, LocatedMappingClause),
  End(LocatedIdentifier),
}

/// ScatteredDefinition with location
pub type LocatedScatteredDefinition = Located<ScatteredDefinition>;

/// Represents loop measure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopMeasure {
  pub loop_type: LoopType,
  pub expression: Box<LocatedExpression>,
}

/// Represents precedence
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Precedence {
  Infix,
  InfixL,
  InfixR,
}

/// Represents fixity token
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixityToken(pub Precedence, pub BigInteger, pub String);

/// Represents top-level definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Definition {
  TypeDefinition(LocatedTypeDefinition),
  Constraint(Box<LocatedAbstractType>),
  FunctionDefinition(LocatedFunctionDefinition),
  MappingDefinition(LocatedMappingDefinition),
  Implementation(LocatedFunctionClause),
  ValueDefinition(LocatedLetBinding),
  Overload(LocatedIdentifier, Vec<LocatedIdentifier>),
  Fixity(Precedence, BigInteger, LocatedIdentifier),
  ValueSpec(LocatedValueSpecification),
  OutcomeSpec(LocatedOutcomeSpec, Vec<LocatedDefinition>),
  Instantiation(LocatedIdentifier, Vec<LocatedInstantiationSubstitution>),
  DefaultTypingSpec(LocatedDefaultTypingSpec),
  ScatteredDefinition(LocatedScatteredDefinition),
  Measure(LocatedIdentifier, Box<LocatedPattern>, Box<LocatedExpression>),
  LoopMeasures(LocatedIdentifier, Vec<LoopMeasure>),
  Register(LocatedDeclarationSpecification),
  Pragma(String, String, i32),
  Private(Box<LocatedDefinition>),
  Attribute(String, Option<LocatedAttributeData>, Box<LocatedDefinition>),
  Documentation(String, Box<LocatedDefinition>),
  InternalMutRec(Vec<LocatedFunctionDefinition>),
}

/// Definition with location
pub type LocatedDefinition = Located<Definition>;

/// Represents lvalue expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LValueExpression {
  Identifier(LocatedIdentifier),
  Memory(LocatedIdentifier, Vec<LocatedExpression>),
  Vector(Box<LocatedLValueExpression>, Box<LocatedExpression>),
  VectorRange(Box<LocatedLValueExpression>, Box<LocatedExpression>, Box<LocatedExpression>),
  VectorConcat(Vec<LocatedLValueExpression>),
  Field(Box<LocatedLValueExpression>, LocatedIdentifier),
}

/// LValueExpression with location
pub type LocatedLValueExpression = Located<LValueExpression>;

/// Represents definition sequence
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definitions(pub Vec<(String, Vec<LocatedDefinition>)>);
