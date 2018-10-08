/* fluent-web@0.0.1 */
/**
 * Below is a manually a list of likely subtags corresponding to Unicode
 * CLDR likelySubtags list.
 * This list is curated by the maintainers of Project Fluent and is
 * intended to be used in place of the full likelySubtags list in use cases
 * where full list cannot be (for example, due to the size).
 *
 * This version of the list is based on CLDR 30.0.3.
 */
const likelySubtagsMin = {
  'ar': 'ar-arab-eg',
  'az-arab': 'az-arab-ir',
  'az-ir': 'az-arab-ir',
  'be': 'be-cyrl-by',
  'da': 'da-latn-dk',
  'el': 'el-grek-gr',
  'en': 'en-latn-us',
  'fa': 'fa-arab-ir',
  'ja': 'ja-jpan-jp',
  'ko': 'ko-kore-kr',
  'pt': 'pt-latn-br',
  'sr': 'sr-cyrl-rs',
  'sr-ru': 'sr-latn-ru',
  'sv': 'sv-latn-se',
  'ta': 'ta-taml-in',
  'uk': 'uk-cyrl-ua',
  'zh': 'zh-hans-cn',
  'zh-gb': 'zh-hant-gb',
  'zh-us': 'zh-hant-us',
};

const regionMatchingLangs = [
  'az',
  'bg',
  'cs',
  'de',
  'es',
  'fi',
  'fr',
  'hu',
  'it',
  'lt',
  'lv',
  'nl',
  'pl',
  'ro',
  'ru',
];

function getLikelySubtagsMin(loc) {
  if (likelySubtagsMin.hasOwnProperty(loc)) {
    return new Locale(likelySubtagsMin[loc]);
  }
  const locale = new Locale(loc);
  if (regionMatchingLangs.includes(locale.language)) {
    locale.region = locale.language;
    locale.string = `${locale.language}-${locale.region}`;
    return locale;
  }
  return null;
}

/* eslint no-magic-numbers: 0 */

const languageCodeRe = '([a-z]{2,3}|\\*)';
const scriptCodeRe = '(?:-([a-z]{4}|\\*))';
const regionCodeRe = '(?:-([a-z]{2}|\\*))';
const variantCodeRe = '(?:-([a-z]{3}|\\*))';

/**
 * Regular expression splitting locale id into four pieces:
 *
 * Example: `en-Latn-US-mac`
 *
 * language: en
 * script:   Latn
 * region:   US
 * variant:  mac
 *
 * It can also accept a range `*` character on any position.
 */
const localeRe = new RegExp(
`^${languageCodeRe}${scriptCodeRe}?${regionCodeRe}?${variantCodeRe}?$`, 'i');

const localeParts = ['language', 'script', 'region', 'variant'];

class Locale {
  /**
   * Parses a locale id using the localeRe into an array with four elements.
   *
   * If the second argument `range` is set to true, it places range `*` char
   * in place of any missing piece.
   *
   * It also allows skipping the script section of the id, so `en-US` is
   * properly parsed as `en-*-US-*`.
   */
  constructor(locale, range = false) {
    const result = localeRe.exec(locale.replace(/_/g, '-'));
    if (!result) {
      return;
    }

    const missing = range ? '*' : undefined;

    const language = result[1] || missing;
    const script = result[2] || missing;
    const region = result[3] || missing;
    const variant = result[4] || missing;

    this.language = language;
    this.script = script;
    this.region = region;
    this.variant = variant;
    this.string = locale;
  }

  isEqual(locale) {
    return localeParts.every(part => this[part] === locale[part]);
  }

  matches(locale) {
    return localeParts.every(part => {
      return this[part] === '*' || locale[part] === '*' ||
        (this[part] === undefined && locale[part] === undefined) ||
        (this[part] !== undefined && locale[part] !== undefined &&
        this[part].toLowerCase() === locale[part].toLowerCase());
    });
  }

  setVariantRange() {
    this.variant = '*';
  }

  setRegionRange() {
    this.region = '*';
  }

  addLikelySubtags() {
    const newLocale = getLikelySubtagsMin(this.string.toLowerCase());

    if (newLocale) {
      localeParts.forEach(part => this[part] = newLocale[part]);
      this.string = newLocale.string;
      return true;
    }
    return false;
  }
}

/* eslint no-magic-numbers: 0 */

/**
 * Negotiates the languages between the list of requested locales against
 * a list of available locales.
 *
 * The algorithm is based on the BCP4647 3.3.2 Extended Filtering algorithm,
 * with several modifications:
 *
 *  1) available locales are treated as ranges
 *
 *    This change allows us to match a more specific request against
 *    more generic available locale.
 *
 *    For example, if the available locale list provides locale `en`,
 *    and the requested locale is `en-US`, we treat the available locale as
 *    a locale that matches all possible english requests.
 *
 *    This means that we expect available locale ID to be as precize as
 *    the matches they want to cover.
 *
 *    For example, if there is only `sr` available, it's ok to list
 *    it in available locales. But once the available locales has both,
 *    Cyrl and Latn variants, the locale IDs should be `sr-Cyrl` and `sr-Latn`
 *    to avoid any `sr-*` request to match against whole `sr` range.
 *
 *    What it does ([requested] * [available] = [supported]):
 *
 *    ['en-US'] * ['en'] = ['en']
 *
 *  2) likely subtags from LDML 4.3 Likely Subtags has been added
 *
 *    The most obvious likely subtag that can be computed is a duplication
 *    of the language field onto region field (`fr` => `fr-FR`).
 *
 *    On top of that, likely subtags may use a list of mappings, that
 *    allow the algorithm to handle non-obvious matches.
 *    For example, making sure that we match `en` to `en-US` or `sr` to
 *    `sr-Cyrl`, while `sr-RU` to `sr-Latn-RU`.
 *
 *    This list can be taken directly from CLDR Supplemental Data.
 *
 *    What it does ([requested] * [available] = [supported]):
 *
 *    ['fr'] * ['fr-FR'] = ['fr-FR']
 *    ['en'] * ['en-US'] = ['en-US']
 *    ['sr'] * ['sr-Latn', 'sr-Cyrl'] = ['sr-Cyrl']
 *
 *  3) variant/region range check has been added
 *
 *    Lastly, the last form of check is against the requested locale ID
 *    but with the variant/region field replaced with a `*` range.
 *
 *    The rationale here laid out in LDML 4.4 Language Matching:
 *      "(...) normally the fall-off between the user's languages is
 *      substantially greated than regional variants."
 *
 *    In other words, if we can't match for the given region, maybe
 *    we can match for the same language/script but other region, and
 *    it will in most cases be preferred over falling back on the next
 *    language.
 *
 *    What it does ([requested] * [available] = [supported]):
 *
 *    ['en-AU'] * ['en-US'] = ['en-US']
 *    ['sr-RU'] * ['sr-Latn-RO'] = ['sr-Latn-RO'] // sr-RU -> sr-Latn-RU
 *
 *    It works similarly to getParentLocales algo, except that we stop
 *    after matching against variant/region ranges and don't try to match
 *    ignoring script ranges. That means that `sr-Cyrl` will never match
 *    against `sr-Latn`.
 */
function filterMatches(
  requestedLocales, availableLocales, strategy
) {
  const supportedLocales = new Set();

  const availLocales =
    new Set(availableLocales.map(locale => new Locale(locale, true)));

  outer:
  for (const reqLocStr of requestedLocales) {
    const reqLocStrLC = reqLocStr.toLowerCase();
    const requestedLocale = new Locale(reqLocStrLC);

    if (requestedLocale.language === undefined) {
      continue;
    }

    // Attempt to make an exact match
    // Example: `en-US` === `en-US`
    for (const availableLocale of availableLocales) {
      if (reqLocStrLC === availableLocale.toLowerCase()) {
        supportedLocales.add(availableLocale);
        for (const loc of availLocales) {
          if (loc.isEqual(requestedLocale)) {
            availLocales.delete(loc);
            break;
          }
        }
        if (strategy === 'lookup') {
          return Array.from(supportedLocales);
        } else if (strategy === 'filtering') {
          continue;
        } else {
          continue outer;
        }
      }
    }


    // Attempt to match against the available range
    // This turns `en` into `en-*-*-*` and `en-US` into `en-*-US-*`
    // Example: ['en-US'] * ['en'] = ['en']
    for (const availableLocale of availLocales) {
      if (requestedLocale.matches(availableLocale)) {
        supportedLocales.add(availableLocale.string);
        availLocales.delete(availableLocale);
        if (strategy === 'lookup') {
          return Array.from(supportedLocales);
        } else if (strategy === 'filtering') {
          continue;
        } else {
          continue outer;
        }
      }
    }

    // Attempt to retrieve a maximal version of the requested locale ID
    // If data is available, it'll expand `en` into `en-Latn-US` and
    // `zh` into `zh-Hans-CN`.
    // Example: ['en'] * ['en-GB', 'en-US'] = ['en-US']
    if (requestedLocale.addLikelySubtags()) {
      for (const availableLocale of availLocales) {
        if (requestedLocale.matches(availableLocale)) {
          supportedLocales.add(availableLocale.string);
          availLocales.delete(availableLocale);
          if (strategy === 'lookup') {
            return Array.from(supportedLocales);
          } else if (strategy === 'filtering') {
            continue;
          } else {
            continue outer;
          }
        }
      }
    }

    // Attempt to look up for a different variant for the same locale ID
    // Example: ['en-US-mac'] * ['en-US-win'] = ['en-US-win']
    requestedLocale.setVariantRange();

    for (const availableLocale of availLocales) {
      if (requestedLocale.matches(availableLocale)) {
        supportedLocales.add(availableLocale.string);
        availLocales.delete(availableLocale);
        if (strategy === 'lookup') {
          return Array.from(supportedLocales);
        } else if (strategy === 'filtering') {
          continue;
        } else {
          continue outer;
        }
      }
    }

    // Attempt to look up for a different region for the same locale ID
    // Example: ['en-US'] * ['en-AU'] = ['en-AU']
    requestedLocale.setRegionRange();

    for (const availableLocale of availLocales) {
      if (requestedLocale.matches(availableLocale)) {
        supportedLocales.add(availableLocale.string);
        availLocales.delete(availableLocale);
        if (strategy === 'lookup') {
          return Array.from(supportedLocales);
        } else if (strategy === 'filtering') {
          continue;
        } else {
          continue outer;
        }
      }
    }
  }

  return Array.from(supportedLocales);
}

function GetOption(options, property, type, values, fallback) {
  let value = options[property];

  if (value !== undefined) {
    if (type === 'boolean') {
      value = new Boolean(value);
    } else if (type === 'string') {
      value = String(value);
    }

    if (values !== undefined && values.indexOf(value) === -1) {
      throw new Error('Invalid option value');
    }

    return value;
  }

  return fallback;
}

/**
 * Negotiates the languages between the list of requested locales against
 * a list of available locales.
 *
 * It accepts three arguments:
 *
 *   requestedLocales:
 *     an Array of strings with BCP47 locale IDs sorted
 *     according to user preferences.
 *
 *   availableLocales:
 *     an Array of strings with BCP47 locale IDs of locale for which
 *     resources are available. Unsorted.
 *
 *   options:
 *     An object with the following, optional keys:
 *
 *       strategy: 'filtering' (default) | 'matching' | 'lookup'
 *
 *       defaultLocale:
 *         a string with BCP47 locale ID to be used
 *         as a last resort locale.
 *
 *       likelySubtags:
 *         a key-value map of locale keys to their most expanded variants.
 *         For example:
 *           'en' -> 'en-Latn-US',
 *           'ru' -> 'ru-Cyrl-RU',
 *
 *
 * It returns an Array of strings with BCP47 locale IDs sorted according to the
 * user preferences.
 *
 * The exact list will be selected differently depending on the strategy:
 *
 *   'filtering': (default)
 *     In the filtering strategy, the algorithm will attempt to match
 *     as many keys in the available locales in order of the requested locales.
 *
 *   'matching':
 *     In the matching strategy, the algorithm will attempt to find the
 *     best possible match for each element of the requestedLocales list.
 *
 *   'lookup':
 *     In the lookup strategy, the algorithm will attempt to find a single
 *     best available locale based on the requested locales list.
 *
 *     This strategy requires defaultLocale option to be set.
 */
function negotiateLanguages(
  requestedLocales,
  availableLocales,
  options = {}
) {

  const defaultLocale = GetOption(options, 'defaultLocale', 'string');
  const likelySubtags = GetOption(
    options, 'likelySubtags', 'object', undefined);
  const strategy = GetOption(options, 'strategy', 'string',
    ['filtering', 'matching', 'lookup'], 'filtering');

  if (strategy === 'lookup' && !defaultLocale) {
    throw new Error('defaultLocale cannot be undefined for strategy `lookup`');
  }

  const resolvedReqLoc = Array.from(Object(requestedLocales)).map(loc => {
    return String(loc);
  });
  const resolvedAvailLoc = Array.from(Object(availableLocales)).map(loc => {
    return String(loc);
  });

  const supportedLocales = filterMatches(
    resolvedReqLoc,
    resolvedAvailLoc, strategy, likelySubtags
  );

  if (strategy === 'lookup') {
    if (supportedLocales.length === 0) {
      supportedLocales.push(defaultLocale);
    }
  } else if (defaultLocale && !supportedLocales.includes(defaultLocale)) {
    supportedLocales.push(defaultLocale);
  }
  return supportedLocales;
}

/*
 * @module fluent-langneg
 * @overview
 *
 * `fluent-langneg` provides language negotiation API that fits into
 * Project Fluent localization composition and fallbacking strategy.
 *
 */

/* global Intl */

/**
 * The `FluentType` class is the base of Fluent's type system.
 *
 * Fluent types wrap JavaScript values and store additional configuration for
 * them, which can then be used in the `toString` method together with a proper
 * `Intl` formatter.
 */
class FluentType {

  /**
   * Create an `FluentType` instance.
   *
   * @param   {Any}    value - JavaScript value to wrap.
   * @param   {Object} opts  - Configuration.
   * @returns {FluentType}
   */
  constructor(value, opts) {
    this.value = value;
    this.opts = opts;
  }

  /**
   * Unwrap the raw value stored by this `FluentType`.
   *
   * @returns {Any}
   */
  valueOf() {
    return this.value;
  }

  /**
   * Format this instance of `FluentType` to a string.
   *
   * Formatted values are suitable for use outside of the `FluentBundle`.
   * This method can use `Intl` formatters memoized by the `FluentBundle`
   * instance passed as an argument.
   *
   * @param   {FluentBundle} [bundle]
   * @returns {string}
   */
  toString() {
    throw new Error("Subclasses of FluentType must implement toString.");
  }
}

class FluentNone extends FluentType {
  toString() {
    return this.value || "???";
  }
}

class FluentNumber extends FluentType {
  constructor(value, opts) {
    super(parseFloat(value), opts);
  }

  toString(bundle) {
    try {
      const nf = bundle._memoizeIntlObject(
        Intl.NumberFormat, this.opts
      );
      return nf.format(this.value);
    } catch (e) {
      // XXX Report the error.
      return this.value;
    }
  }

  /**
   * Compare the object with another instance of a FluentType.
   *
   * @param   {FluentBundle} bundle
   * @param   {FluentType}     other
   * @returns {bool}
   */
  match(bundle, other) {
    if (other instanceof FluentNumber) {
      return this.value === other.value;
    }
    return false;
  }
}

class FluentDateTime extends FluentType {
  constructor(value, opts) {
    super(new Date(value), opts);
  }

  toString(bundle) {
    try {
      const dtf = bundle._memoizeIntlObject(
        Intl.DateTimeFormat, this.opts
      );
      return dtf.format(this.value);
    } catch (e) {
      // XXX Report the error.
      return this.value;
    }
  }
}

class FluentSymbol extends FluentType {
  toString() {
    return this.value;
  }

  /**
   * Compare the object with another instance of a FluentType.
   *
   * @param   {FluentBundle} bundle
   * @param   {FluentType}     other
   * @returns {bool}
   */
  match(bundle, other) {
    if (other instanceof FluentSymbol) {
      return this.value === other.value;
    } else if (typeof other === "string") {
      return this.value === other;
    } else if (other instanceof FluentNumber) {
      const pr = bundle._memoizeIntlObject(
        Intl.PluralRules, other.opts
      );
      return this.value === pr.select(other.value);
    }
    return false;
  }
}

/**
 * @overview
 *
 * The FTL resolver ships with a number of functions built-in.
 *
 * Each function take two arguments:
 *   - args - an array of positional args
 *   - opts - an object of key-value args
 *
 * Arguments to functions are guaranteed to already be instances of
 * `FluentType`.  Functions must return `FluentType` objects as well.
 */

var builtins = {
  "NUMBER": ([arg], opts) =>
    new FluentNumber(arg.valueOf(), merge(arg.opts, opts)),
  "DATETIME": ([arg], opts) =>
    new FluentDateTime(arg.valueOf(), merge(arg.opts, opts)),
};

function merge(argopts, opts) {
  return Object.assign({}, argopts, values(opts));
}

function values(opts) {
  const unwrapped = {};
  for (const [name, opt] of Object.entries(opts)) {
    unwrapped[name] = opt.valueOf();
  }
  return unwrapped;
}

/**
 * @overview
 *
 * The role of the Fluent resolver is to format a translation object to an
 * instance of `FluentType` or an array of instances.
 *
 * Translations can contain references to other messages or variables,
 * conditional logic in form of select expressions, traits which describe their
 * grammatical features, and can use Fluent builtins which make use of the
 * `Intl` formatters to format numbers, dates, lists and more into the
 * bundle's language. See the documentation of the Fluent syntax for more
 * information.
 *
 * In case of errors the resolver will try to salvage as much of the
 * translation as possible.  In rare situations where the resolver didn't know
 * how to recover from an error it will return an instance of `FluentNone`.
 *
 * `MessageReference`, `VariantExpression`, `AttributeExpression` and
 * `SelectExpression` resolve to raw Runtime Entries objects and the result of
 * the resolution needs to be passed into `Type` to get their real value.
 * This is useful for composing expressions.  Consider:
 *
 *     brand-name[nominative]
 *
 * which is a `VariantExpression` with properties `id: MessageReference` and
 * `key: Keyword`.  If `MessageReference` was resolved eagerly, it would
 * instantly resolve to the value of the `brand-name` message.  Instead, we
 * want to get the message object and look for its `nominative` variant.
 *
 * All other expressions (except for `FunctionReference` which is only used in
 * `CallExpression`) resolve to an instance of `FluentType`.  The caller should
 * use the `toString` method to convert the instance to a native value.
 *
 *
 * All functions in this file pass around a special object called `env`.
 * This object stores a set of elements used by all resolve functions:
 *
 *  * {FluentBundle} bundle
 *      bundle for which the given resolution is happening
 *  * {Object} args
 *      list of developer provided arguments that can be used
 *  * {Array} errors
 *      list of errors collected while resolving
 *  * {WeakSet} dirty
 *      Set of patterns already encountered during this resolution.
 *      This is used to prevent cyclic resolutions.
 */

// Prevent expansion of too long placeables.
const MAX_PLACEABLE_LENGTH = 2500;

// Unicode bidi isolation characters.
const FSI = "\u2068";
const PDI = "\u2069";


/**
 * Helper for choosing the default value from a set of members.
 *
 * Used in SelectExpressions and Type.
 *
 * @param   {Object} env
 *    Resolver environment object.
 * @param   {Object} members
 *    Hash map of variants from which the default value is to be selected.
 * @param   {Number} def
 *    The index of the default variant.
 * @returns {FluentType}
 * @private
 */
function DefaultMember(env, members, def) {
  if (members[def]) {
    return members[def];
  }

  const { errors } = env;
  errors.push(new RangeError("No default"));
  return new FluentNone();
}


/**
 * Resolve a reference to another message.
 *
 * @param   {Object} env
 *    Resolver environment object.
 * @param   {Object} id
 *    The identifier of the message to be resolved.
 * @param   {String} id.name
 *    The name of the identifier.
 * @returns {FluentType}
 * @private
 */
function MessageReference(env, {name}) {
  const { bundle, errors } = env;
  const message = name.startsWith("-")
    ? bundle._terms.get(name)
    : bundle._messages.get(name);

  if (!message) {
    const err = name.startsWith("-")
      ? new ReferenceError(`Unknown term: ${name}`)
      : new ReferenceError(`Unknown message: ${name}`);
    errors.push(err);
    return new FluentNone(name);
  }

  return message;
}

/**
 * Resolve a variant expression to the variant object.
 *
 * @param   {Object} env
 *    Resolver environment object.
 * @param   {Object} expr
 *    An expression to be resolved.
 * @param   {Object} expr.id
 *    An Identifier of a message for which the variant is resolved.
 * @param   {Object} expr.id.name
 *    Name a message for which the variant is resolved.
 * @param   {Object} expr.key
 *    Variant key to be resolved.
 * @returns {FluentType}
 * @private
 */
function VariantExpression(env, {id, key}) {
  const message = MessageReference(env, id);
  if (message instanceof FluentNone) {
    return message;
  }

  const { bundle, errors } = env;
  const keyword = Type(env, key);

  function isVariantList(node) {
    return Array.isArray(node) &&
      node[0].type === "sel" &&
      node[0].exp === null;
  }

  if (isVariantList(message.val)) {
    // Match the specified key against keys of each variant, in order.
    for (const variant of message.val[0].vars) {
      const variantKey = Type(env, variant.key);
      if (keyword.match(bundle, variantKey)) {
        return variant;
      }
    }
  }

  errors.push(
    new ReferenceError(`Unknown variant: ${keyword.toString(bundle)}`));
  return Type(env, message);
}


/**
 * Resolve an attribute expression to the attribute object.
 *
 * @param   {Object} env
 *    Resolver environment object.
 * @param   {Object} expr
 *    An expression to be resolved.
 * @param   {String} expr.id
 *    An ID of a message for which the attribute is resolved.
 * @param   {String} expr.name
 *    Name of the attribute to be resolved.
 * @returns {FluentType}
 * @private
 */
function AttributeExpression(env, {id, name}) {
  const message = MessageReference(env, id);
  if (message instanceof FluentNone) {
    return message;
  }

  if (message.attrs) {
    // Match the specified name against keys of each attribute.
    for (const attrName in message.attrs) {
      if (name === attrName) {
        return message.attrs[name];
      }
    }
  }

  const { errors } = env;
  errors.push(new ReferenceError(`Unknown attribute: ${name}`));
  return Type(env, message);
}

/**
 * Resolve a select expression to the member object.
 *
 * @param   {Object} env
 *    Resolver environment object.
 * @param   {Object} expr
 *    An expression to be resolved.
 * @param   {String} expr.exp
 *    Selector expression
 * @param   {Array} expr.vars
 *    List of variants for the select expression.
 * @param   {Number} expr.def
 *    Index of the default variant.
 * @returns {FluentType}
 * @private
 */
function SelectExpression(env, {exp, vars, def}) {
  if (exp === null) {
    return DefaultMember(env, vars, def);
  }

  const selector = Type(env, exp);
  if (selector instanceof FluentNone) {
    return DefaultMember(env, vars, def);
  }

  // Match the selector against keys of each variant, in order.
  for (const variant of vars) {
    const key = Type(env, variant.key);
    const keyCanMatch =
      key instanceof FluentNumber || key instanceof FluentSymbol;

    if (!keyCanMatch) {
      continue;
    }

    const { bundle } = env;

    if (key.match(bundle, selector)) {
      return variant;
    }
  }

  return DefaultMember(env, vars, def);
}


/**
 * Resolve expression to a Fluent type.
 *
 * JavaScript strings are a special case.  Since they natively have the
 * `toString` method they can be used as if they were a Fluent type without
 * paying the cost of creating a instance of one.
 *
 * @param   {Object} env
 *    Resolver environment object.
 * @param   {Object} expr
 *    An expression object to be resolved into a Fluent type.
 * @returns {FluentType}
 * @private
 */
function Type(env, expr) {
  // A fast-path for strings which are the most common case, and for
  // `FluentNone` which doesn't require any additional logic.
  if (typeof expr === "string") {
    return env.bundle._transform(expr);
  }
  if (expr instanceof FluentNone) {
    return expr;
  }

  // The Runtime AST (Entries) encodes patterns (complex strings with
  // placeables) as Arrays.
  if (Array.isArray(expr)) {
    return Pattern(env, expr);
  }


  switch (expr.type) {
    case "varname":
      return new FluentSymbol(expr.name);
    case "num":
      return new FluentNumber(expr.val);
    case "var":
      return VariableReference(env, expr);
    case "fun":
      return FunctionReference(env, expr);
    case "call":
      return CallExpression(env, expr);
    case "ref": {
      const message = MessageReference(env, expr);
      return Type(env, message);
    }
    case "getattr": {
      const attr = AttributeExpression(env, expr);
      return Type(env, attr);
    }
    case "getvar": {
      const variant = VariantExpression(env, expr);
      return Type(env, variant);
    }
    case "sel": {
      const member = SelectExpression(env, expr);
      return Type(env, member);
    }
    case undefined: {
      // If it's a node with a value, resolve the value.
      if (expr.val !== null && expr.val !== undefined) {
        return Type(env, expr.val);
      }

      const { errors } = env;
      errors.push(new RangeError("No value"));
      return new FluentNone();
    }
    default:
      return new FluentNone();
  }
}

/**
 * Resolve a reference to a variable.
 *
 * @param   {Object} env
 *    Resolver environment object.
 * @param   {Object} expr
 *    An expression to be resolved.
 * @param   {String} expr.name
 *    Name of an argument to be returned.
 * @returns {FluentType}
 * @private
 */
function VariableReference(env, {name}) {
  const { args, errors } = env;

  if (!args || !args.hasOwnProperty(name)) {
    errors.push(new ReferenceError(`Unknown variable: ${name}`));
    return new FluentNone(name);
  }

  const arg = args[name];

  // Return early if the argument already is an instance of FluentType.
  if (arg instanceof FluentType) {
    return arg;
  }

  // Convert the argument to a Fluent type.
  switch (typeof arg) {
    case "string":
      return arg;
    case "number":
      return new FluentNumber(arg);
    case "object":
      if (arg instanceof Date) {
        return new FluentDateTime(arg);
      }
    default:
      errors.push(
        new TypeError(`Unsupported variable type: ${name}, ${typeof arg}`)
      );
      return new FluentNone(name);
  }
}

/**
 * Resolve a reference to a function.
 *
 * @param   {Object}  env
 *    Resolver environment object.
 * @param   {Object} expr
 *    An expression to be resolved.
 * @param   {String} expr.name
 *    Name of the function to be returned.
 * @returns {Function}
 * @private
 */
function FunctionReference(env, {name}) {
  // Some functions are built-in.  Others may be provided by the runtime via
  // the `FluentBundle` constructor.
  const { bundle: { _functions }, errors } = env;
  const func = _functions[name] || builtins[name];

  if (!func) {
    errors.push(new ReferenceError(`Unknown function: ${name}()`));
    return new FluentNone(`${name}()`);
  }

  if (typeof func !== "function") {
    errors.push(new TypeError(`Function ${name}() is not callable`));
    return new FluentNone(`${name}()`);
  }

  return func;
}

/**
 * Resolve a call to a Function with positional and key-value arguments.
 *
 * @param   {Object} env
 *    Resolver environment object.
 * @param   {Object} expr
 *    An expression to be resolved.
 * @param   {Object} expr.fun
 *    FTL Function object.
 * @param   {Array} expr.args
 *    FTL Function argument list.
 * @returns {FluentType}
 * @private
 */
function CallExpression(env, {fun, args}) {
  const callee = FunctionReference(env, fun);

  if (callee instanceof FluentNone) {
    return callee;
  }

  const posargs = [];
  const keyargs = {};

  for (const arg of args) {
    if (arg.type === "narg") {
      keyargs[arg.name] = Type(env, arg.val);
    } else {
      posargs.push(Type(env, arg));
    }
  }

  try {
    return callee(posargs, keyargs);
  } catch (e) {
    // XXX Report errors.
    return new FluentNone();
  }
}

/**
 * Resolve a pattern (a complex string with placeables).
 *
 * @param   {Object} env
 *    Resolver environment object.
 * @param   {Array} ptn
 *    Array of pattern elements.
 * @returns {Array}
 * @private
 */
function Pattern(env, ptn) {
  const { bundle, dirty, errors } = env;

  if (dirty.has(ptn)) {
    errors.push(new RangeError("Cyclic reference"));
    return new FluentNone();
  }

  // Tag the pattern as dirty for the purpose of the current resolution.
  dirty.add(ptn);
  const result = [];

  // Wrap interpolations with Directional Isolate Formatting characters
  // only when the pattern has more than one element.
  const useIsolating = bundle._useIsolating && ptn.length > 1;

  for (const elem of ptn) {
    if (typeof elem === "string") {
      result.push(bundle._transform(elem));
      continue;
    }

    const part = Type(env, elem).toString(bundle);

    if (useIsolating) {
      result.push(FSI);
    }

    if (part.length > MAX_PLACEABLE_LENGTH) {
      errors.push(
        new RangeError(
          "Too many characters in placeable " +
          `(${part.length}, max allowed is ${MAX_PLACEABLE_LENGTH})`
        )
      );
      result.push(part.slice(MAX_PLACEABLE_LENGTH));
    } else {
      result.push(part);
    }

    if (useIsolating) {
      result.push(PDI);
    }
  }

  dirty.delete(ptn);
  return result.join("");
}

/**
 * Format a translation into a string.
 *
 * @param   {FluentBundle} bundle
 *    A FluentBundle instance which will be used to resolve the
 *    contextual information of the message.
 * @param   {Object}         args
 *    List of arguments provided by the developer which can be accessed
 *    from the message.
 * @param   {Object}         message
 *    An object with the Message to be resolved.
 * @param   {Array}          errors
 *    An error array that any encountered errors will be appended to.
 * @returns {FluentType}
 */
function resolve(bundle, args, message, errors = []) {
  const env = {
    bundle, args, errors, dirty: new WeakSet()
  };
  return Type(env, message).toString(bundle);
}

/*  eslint no-magic-numbers: [0]  */

const MAX_PLACEABLES = 100;

const entryIdentifierRe = /-?[a-zA-Z][a-zA-Z0-9_-]*/y;
const identifierRe = /[a-zA-Z][a-zA-Z0-9_-]*/y;
const functionIdentifierRe = /^[A-Z][A-Z_?-]*$/;
const unicodeEscapeRe = /^[a-fA-F0-9]{4}$/;
const trailingWSRe = /[ \t\n\r]+$/;


/**
 * The `Parser` class is responsible for parsing FTL resources.
 *
 * It's only public method is `getResource(source)` which takes an FTL string
 * and returns a two element Array with an Object of entries generated from the
 * source as the first element and an array of SyntaxError objects as the
 * second.
 *
 * This parser is optimized for runtime performance.
 *
 * There is an equivalent of this parser in syntax/parser which is
 * generating full AST which is useful for FTL tools.
 */
class RuntimeParser {
  /**
   * Parse FTL code into entries formattable by the FluentBundle.
   *
   * Given a string of FTL syntax, return a map of entries that can be passed
   * to FluentBundle.format and a list of errors encountered during parsing.
   *
   * @param {String} string
   * @returns {Array<Object, Array>}
   */
  getResource(string) {
    this._source = string;
    this._index = 0;
    this._length = string.length;
    this.entries = {};

    const errors = [];

    this.skipWS();
    while (this._index < this._length) {
      try {
        this.getEntry();
      } catch (e) {
        if (e instanceof SyntaxError) {
          errors.push(e);

          this.skipToNextEntryStart();
        } else {
          throw e;
        }
      }
      this.skipWS();
    }

    return [this.entries, errors];
  }

  /**
   * Parse the source string from the current index as an FTL entry
   * and add it to object's entries property.
   *
   * @private
   */
  getEntry() {
    // The index here should either be at the beginning of the file
    // or right after new line.
    if (this._index !== 0 &&
        this._source[this._index - 1] !== "\n") {
      throw this.error(`Expected an entry to start
        at the beginning of the file or on a new line.`);
    }

    const ch = this._source[this._index];

    // We don't care about comments or sections at runtime
    if (ch === "#" &&
        [" ", "#", "\n"].includes(this._source[this._index + 1])) {
      this.skipComment();
      return;
    }

    this.getMessage();
  }

  /**
   * Parse the source string from the current index as an FTL message
   * and add it to the entries property on the Parser.
   *
   * @private
   */
  getMessage() {
    const id = this.getEntryIdentifier();

    this.skipInlineWS();

    if (this._source[this._index] === "=") {
      this._index++;
    } else {
      throw this.error("Expected \"=\" after the identifier");
    }

    this.skipInlineWS();

    const val = this.getPattern();

    if (id.startsWith("-") && val === null) {
      throw this.error("Expected term to have a value");
    }

    let attrs = null;

    if (this._source[this._index] === " ") {
      const lineStart = this._index;
      this.skipInlineWS();

      if (this._source[this._index] === ".") {
        this._index = lineStart;
        attrs = this.getAttributes();
      }
    }

    if (attrs === null && typeof val === "string") {
      this.entries[id] = val;
    } else {
      if (val === null && attrs === null) {
        throw this.error("Expected message to have a value or attributes");
      }

      this.entries[id] = {};

      if (val !== null) {
        this.entries[id].val = val;
      }

      if (attrs !== null) {
        this.entries[id].attrs = attrs;
      }
    }
  }

  /**
   * Skip whitespace.
   *
   * @private
   */
  skipWS() {
    let ch = this._source[this._index];
    while (ch === " " || ch === "\n" || ch === "\t" || ch === "\r") {
      ch = this._source[++this._index];
    }
  }

  /**
   * Skip inline whitespace (space and \t).
   *
   * @private
   */
  skipInlineWS() {
    let ch = this._source[this._index];
    while (ch === " " || ch === "\t") {
      ch = this._source[++this._index];
    }
  }

  /**
   * Skip blank lines.
   *
   * @private
   */
  skipBlankLines() {
    while (true) {
      const ptr = this._index;

      this.skipInlineWS();

      if (this._source[this._index] === "\n") {
        this._index += 1;
      } else {
        this._index = ptr;
        break;
      }
    }
  }

  /**
   * Get identifier using the provided regex.
   *
   * By default this will get identifiers of public messages, attributes and
   * variables (without the $).
   *
   * @returns {String}
   * @private
   */
  getIdentifier(re = identifierRe) {
    re.lastIndex = this._index;
    const result = re.exec(this._source);

    if (result === null) {
      this._index += 1;
      throw this.error(`Expected an identifier [${re.toString()}]`);
    }

    this._index = re.lastIndex;
    return result[0];
  }

  /**
   * Get identifier of a Message or a Term (staring with a dash).
   *
   * @returns {String}
   * @private
   */
  getEntryIdentifier() {
    return this.getIdentifier(entryIdentifierRe);
  }

  /**
   * Get Variant name.
   *
   * @returns {Object}
   * @private
   */
  getVariantName() {
    let name = "";

    const start = this._index;
    let cc = this._source.charCodeAt(this._index);

    if ((cc >= 97 && cc <= 122) || // a-z
        (cc >= 65 && cc <= 90) || // A-Z
        cc === 95 || cc === 32) { // _ <space>
      cc = this._source.charCodeAt(++this._index);
    } else {
      throw this.error("Expected a keyword (starting with [a-zA-Z_])");
    }

    while ((cc >= 97 && cc <= 122) || // a-z
           (cc >= 65 && cc <= 90) || // A-Z
           (cc >= 48 && cc <= 57) || // 0-9
           cc === 95 || cc === 45 || cc === 32) { // _- <space>
      cc = this._source.charCodeAt(++this._index);
    }

    // If we encountered the end of name, we want to test if the last
    // collected character is a space.
    // If it is, we will backtrack to the last non-space character because
    // the keyword cannot end with a space character.
    while (this._source.charCodeAt(this._index - 1) === 32) {
      this._index--;
    }

    name += this._source.slice(start, this._index);

    return { type: "varname", name };
  }

  /**
   * Get simple string argument enclosed in `"`.
   *
   * @returns {String}
   * @private
   */
  getString() {
    let value = "";
    this._index++;

    while (this._index < this._length) {
      const ch = this._source[this._index];

      if (ch === '"') {
        this._index++;
        break;
      }

      if (ch === "\n") {
        throw this.error("Unterminated string expression");
      }

      if (ch === "\\") {
        value += this.getEscapedCharacter(["{", "\\", "\""]);
      } else {
        this._index++;
        value += ch;
      }
    }

    return value;
  }

  /**
   * Parses a Message pattern.
   * Message Pattern may be a simple string or an array of strings
   * and placeable expressions.
   *
   * @returns {String|Array}
   * @private
   */
  getPattern() {
    // We're going to first try to see if the pattern is simple.
    // If it is we can just look for the end of the line and read the string.
    //
    // Then, if either the line contains a placeable opening `{` or the
    // next line starts an indentation, we switch to complex pattern.
    const start = this._index;
    let eol = this._source.indexOf("\n", this._index);

    if (eol === -1) {
      eol = this._length;
    }

    // If there's any text between the = and the EOL, store it for now. The next
    // non-empty line will decide what to do with it.
    const firstLineContent = start !== eol
      // Trim the trailing whitespace in case this is a single-line pattern.
      // Multiline patterns are parsed anew by getComplexPattern.
      ? this._source.slice(start, eol).replace(trailingWSRe, "")
      : null;

    if (firstLineContent
      && (firstLineContent.includes("{")
        || firstLineContent.includes("\\"))) {
      return this.getComplexPattern();
    }

    this._index = eol + 1;

    this.skipBlankLines();

    if (this._source[this._index] !== " ") {
      // No indentation means we're done with this message. Callers should check
      // if the return value here is null. It may be OK for messages, but not OK
      // for terms, attributes and variants.
      return firstLineContent;
    }

    const lineStart = this._index;

    this.skipInlineWS();

    if (this._source[this._index] === ".") {
      // The pattern is followed by an attribute. Rewind _index to the first
      // column of the current line as expected by getAttributes.
      this._index = lineStart;
      return firstLineContent;
    }

    if (firstLineContent) {
      // It's a multiline pattern which started on the same line as the
      // identifier. Reparse the whole pattern to make sure we get all of it.
      this._index = start;
    }

    return this.getComplexPattern();
  }

  /**
   * Parses a complex Message pattern.
   * This function is called by getPattern when the message is multiline,
   * or contains escape chars or placeables.
   * It does full parsing of complex patterns.
   *
   * @returns {Array}
   * @private
   */
  /* eslint-disable complexity */
  getComplexPattern() {
    let buffer = "";
    const content = [];
    let placeables = 0;

    let ch = this._source[this._index];

    while (this._index < this._length) {
      // This block handles multi-line strings combining strings separated
      // by new line.
      if (ch === "\n") {
        this._index++;

        // We want to capture the start and end pointers
        // around blank lines and add them to the buffer
        // but only if the blank lines are in the middle
        // of the string.
        const blankLinesStart = this._index;
        this.skipBlankLines();
        const blankLinesEnd = this._index;


        if (this._source[this._index] !== " ") {
          break;
        }
        this.skipInlineWS();

        if (this._source[this._index] === "}" ||
            this._source[this._index] === "[" ||
            this._source[this._index] === "*" ||
            this._source[this._index] === ".") {
          this._index = blankLinesEnd;
          break;
        }

        buffer += this._source.substring(blankLinesStart, blankLinesEnd);

        if (buffer.length || content.length) {
          buffer += "\n";
        }
        ch = this._source[this._index];
        continue;
      }

      if (ch === undefined) {
        break;
      }

      if (ch === "\\") {
        buffer += this.getEscapedCharacter();
        ch = this._source[this._index];
        continue;
      }

      if (ch === "{") {
        // Push the buffer to content array right before placeable
        if (buffer.length) {
          content.push(buffer);
        }
        if (placeables > MAX_PLACEABLES - 1) {
          throw this.error(
            `Too many placeables, maximum allowed is ${MAX_PLACEABLES}`);
        }
        buffer = "";
        content.push(this.getPlaceable());

        ch = this._source[++this._index];
        placeables++;
        continue;
      }

      buffer += ch;
      ch = this._source[++this._index];
    }

    if (content.length === 0) {
      return buffer.length ? buffer : null;
    }

    if (buffer.length) {
      // Trim trailing whitespace, too.
      content.push(buffer.replace(trailingWSRe, ""));
    }

    return content;
  }
  /* eslint-enable complexity */

  /**
   * Parse an escape sequence and return the unescaped character.
   *
   * @returns {string}
   * @private
   */
  getEscapedCharacter(specials = ["{", "\\"]) {
    this._index++;
    const next = this._source[this._index];

    if (specials.includes(next)) {
      this._index++;
      return next;
    }

    if (next === "u") {
      const sequence = this._source.slice(this._index + 1, this._index + 5);
      if (unicodeEscapeRe.test(sequence)) {
        this._index += 5;
        return String.fromCodePoint(parseInt(sequence, 16));
      }

      throw this.error(`Invalid Unicode escape sequence: \\u${sequence}`);
    }

    throw this.error(`Unknown escape sequence: \\${next}`);
  }

  /**
   * Parses a single placeable in a Message pattern and returns its
   * expression.
   *
   * @returns {Object}
   * @private
   */
  getPlaceable() {
    const start = ++this._index;

    this.skipWS();

    if (this._source[this._index] === "*" ||
       (this._source[this._index] === "[" &&
        this._source[this._index + 1] !== "]")) {
      const variants = this.getVariants();

      return {
        type: "sel",
        exp: null,
        vars: variants[0],
        def: variants[1]
      };
    }

    // Rewind the index and only support in-line white-space now.
    this._index = start;
    this.skipInlineWS();

    const selector = this.getSelectorExpression();

    this.skipWS();

    const ch = this._source[this._index];

    if (ch === "}") {
      if (selector.type === "getattr" && selector.id.name.startsWith("-")) {
        throw this.error(
          "Attributes of private messages cannot be interpolated."
        );
      }

      return selector;
    }

    if (ch !== "-" || this._source[this._index + 1] !== ">") {
      throw this.error('Expected "}" or "->"');
    }

    if (selector.type === "ref") {
      throw this.error("Message references cannot be used as selectors.");
    }

    if (selector.type === "getvar") {
      throw this.error("Variants cannot be used as selectors.");
    }

    if (selector.type === "getattr" && !selector.id.name.startsWith("-")) {
      throw this.error(
        "Attributes of public messages cannot be used as selectors."
      );
    }


    this._index += 2; // ->

    this.skipInlineWS();

    if (this._source[this._index] !== "\n") {
      throw this.error("Variants should be listed in a new line");
    }

    this.skipWS();

    const variants = this.getVariants();

    if (variants[0].length === 0) {
      throw this.error("Expected members for the select expression");
    }

    return {
      type: "sel",
      exp: selector,
      vars: variants[0],
      def: variants[1]
    };
  }

  /**
   * Parses a selector expression.
   *
   * @returns {Object}
   * @private
   */
  getSelectorExpression() {
    if (this._source[this._index] === "{") {
      return this.getPlaceable();
    }

    const literal = this.getLiteral();

    if (literal.type !== "ref") {
      return literal;
    }

    if (this._source[this._index] === ".") {
      this._index++;

      const name = this.getIdentifier();
      this._index++;
      return {
        type: "getattr",
        id: literal,
        name
      };
    }

    if (this._source[this._index] === "[") {
      this._index++;

      const key = this.getVariantKey();
      this._index++;
      return {
        type: "getvar",
        id: literal,
        key
      };
    }

    if (this._source[this._index] === "(") {
      this._index++;
      const args = this.getCallArgs();

      if (!functionIdentifierRe.test(literal.name)) {
        throw this.error("Function names must be all upper-case");
      }

      this._index++;

      literal.type = "fun";

      return {
        type: "call",
        fun: literal,
        args
      };
    }

    return literal;
  }

  /**
   * Parses call arguments for a CallExpression.
   *
   * @returns {Array}
   * @private
   */
  getCallArgs() {
    const args = [];

    while (this._index < this._length) {
      this.skipWS();

      if (this._source[this._index] === ")") {
        return args;
      }

      const exp = this.getSelectorExpression();

      // MessageReference in this place may be an entity reference, like:
      // `call(foo)`, or, if it's followed by `:` it will be a key-value pair.
      if (exp.type !== "ref") {
        args.push(exp);
      } else {
        this.skipInlineWS();

        if (this._source[this._index] === ":") {
          this._index++;
          this.skipWS();

          const val = this.getSelectorExpression();

          // If the expression returned as a value of the argument
          // is not a quote delimited string or number, throw.
          //
          // We don't have to check here if the pattern is quote delimited
          // because that's the only type of string allowed in expressions.
          if (typeof val === "string" ||
              Array.isArray(val) ||
              val.type === "num") {
            args.push({
              type: "narg",
              name: exp.name,
              val
            });
          } else {
            this._index = this._source.lastIndexOf(":", this._index) + 1;
            throw this.error(
              "Expected string in quotes, number.");
          }

        } else {
          args.push(exp);
        }
      }

      this.skipWS();

      if (this._source[this._index] === ")") {
        break;
      } else if (this._source[this._index] === ",") {
        this._index++;
      } else {
        throw this.error('Expected "," or ")"');
      }
    }

    return args;
  }

  /**
   * Parses an FTL Number.
   *
   * @returns {Object}
   * @private
   */
  getNumber() {
    let num = "";
    let cc = this._source.charCodeAt(this._index);

    // The number literal may start with negative sign `-`.
    if (cc === 45) {
      num += "-";
      cc = this._source.charCodeAt(++this._index);
    }

    // next, we expect at least one digit
    if (cc < 48 || cc > 57) {
      throw this.error(`Unknown literal "${num}"`);
    }

    // followed by potentially more digits
    while (cc >= 48 && cc <= 57) {
      num += this._source[this._index++];
      cc = this._source.charCodeAt(this._index);
    }

    // followed by an optional decimal separator `.`
    if (cc === 46) {
      num += this._source[this._index++];
      cc = this._source.charCodeAt(this._index);

      // followed by at least one digit
      if (cc < 48 || cc > 57) {
        throw this.error(`Unknown literal "${num}"`);
      }

      // and optionally more digits
      while (cc >= 48 && cc <= 57) {
        num += this._source[this._index++];
        cc = this._source.charCodeAt(this._index);
      }
    }

    return {
      type: "num",
      val: num
    };
  }

  /**
   * Parses a list of Message attributes.
   *
   * @returns {Object}
   * @private
   */
  getAttributes() {
    const attrs = {};

    while (this._index < this._length) {
      if (this._source[this._index] !== " ") {
        break;
      }
      this.skipInlineWS();

      if (this._source[this._index] !== ".") {
        break;
      }
      this._index++;

      const key = this.getIdentifier();

      this.skipInlineWS();

      if (this._source[this._index] !== "=") {
        throw this.error('Expected "="');
      }
      this._index++;

      this.skipInlineWS();

      const val = this.getPattern();

      if (val === null) {
        throw this.error("Expected attribute to have a value");
      }

      if (typeof val === "string") {
        attrs[key] = val;
      } else {
        attrs[key] = {
          val
        };
      }

      this.skipBlankLines();
    }

    return attrs;
  }

  /**
   * Parses a list of Selector variants.
   *
   * @returns {Array}
   * @private
   */
  getVariants() {
    const variants = [];
    let index = 0;
    let defaultIndex;

    while (this._index < this._length) {
      const ch = this._source[this._index];

      if ((ch !== "[" || this._source[this._index + 1] === "[") &&
          ch !== "*") {
        break;
      }
      if (ch === "*") {
        this._index++;
        defaultIndex = index;
      }

      if (this._source[this._index] !== "[") {
        throw this.error('Expected "["');
      }

      this._index++;

      const key = this.getVariantKey();

      this.skipInlineWS();

      const val = this.getPattern();

      if (val === null) {
        throw this.error("Expected variant to have a value");
      }

      variants[index++] = {key, val};

      this.skipWS();
    }

    return [variants, defaultIndex];
  }

  /**
   * Parses a Variant key.
   *
   * @returns {String}
   * @private
   */
  getVariantKey() {
    // VariantKey may be a Keyword or Number

    const cc = this._source.charCodeAt(this._index);
    let literal;

    if ((cc >= 48 && cc <= 57) || cc === 45) {
      literal = this.getNumber();
    } else {
      literal = this.getVariantName();
    }

    if (this._source[this._index] !== "]") {
      throw this.error('Expected "]"');
    }

    this._index++;
    return literal;
  }

  /**
   * Parses an FTL literal.
   *
   * @returns {Object}
   * @private
   */
  getLiteral() {
    const cc0 = this._source.charCodeAt(this._index);

    if (cc0 === 36) { // $
      this._index++;
      return {
        type: "var",
        name: this.getIdentifier()
      };
    }

    const cc1 = cc0 === 45 // -
      // Peek at the next character after the dash.
      ? this._source.charCodeAt(this._index + 1)
      // Or keep using the character at the current index.
      : cc0;

    if ((cc1 >= 97 && cc1 <= 122) || // a-z
        (cc1 >= 65 && cc1 <= 90)) { // A-Z
      return {
        type: "ref",
        name: this.getEntryIdentifier()
      };
    }

    if ((cc1 >= 48 && cc1 <= 57)) { // 0-9
      return this.getNumber();
    }

    if (cc0 === 34) { // "
      return this.getString();
    }

    throw this.error("Expected literal");
  }

  /**
   * Skips an FTL comment.
   *
   * @private
   */
  skipComment() {
    // At runtime, we don't care about comments so we just have
    // to parse them properly and skip their content.
    let eol = this._source.indexOf("\n", this._index);

    while (eol !== -1
      && this._source[eol + 1] === "#"
      && [" ", "#"].includes(this._source[eol + 2])) {

      this._index = eol + 3;
      eol = this._source.indexOf("\n", this._index);

      if (eol === -1) {
        break;
      }
    }

    if (eol === -1) {
      this._index = this._length;
    } else {
      this._index = eol + 1;
    }
  }

  /**
   * Creates a new SyntaxError object with a given message.
   *
   * @param {String} message
   * @returns {Object}
   * @private
   */
  error(message) {
    return new SyntaxError(message);
  }

  /**
   * Skips to the beginning of a next entry after the current position.
   * This is used to mark the boundary of junk entry in case of error,
   * and recover from the returned position.
   *
   * @private
   */
  skipToNextEntryStart() {
    let start = this._index;

    while (true) {
      if (start === 0 || this._source[start - 1] === "\n") {
        const cc = this._source.charCodeAt(start);

        if ((cc >= 97 && cc <= 122) || // a-z
            (cc >= 65 && cc <= 90) || // A-Z
             cc === 45) { // -
          this._index = start;
          return;
        }
      }

      start = this._source.indexOf("\n", start);

      if (start === -1) {
        this._index = this._length;
        return;
      }
      start++;
    }
  }
}

/**
 * Parses an FTL string using RuntimeParser and returns the generated
 * object with entries and a list of errors.
 *
 * @param {String} string
 * @returns {Array<Object, Array>}
 */
function parse(string) {
  const parser = new RuntimeParser();
  return parser.getResource(string);
}

/**
 * Fluent Resource is a structure storing a map
 * of localization entries.
 */
class FluentResource extends Map {
  constructor(entries, errors = []) {
    super(entries);
    this.errors = errors;
  }

  static fromString(source) {
    const [entries, errors] = parse(source);
    return new FluentResource(Object.entries(entries), errors);
  }
}

/**
 * Message bundles are single-language stores of translations.  They are
 * responsible for parsing translation resources in the Fluent syntax and can
 * format translation units (entities) to strings.
 *
 * Always use `FluentBundle.format` to retrieve translation units from a
 * bundle. Translations can contain references to other entities or variables,
 * conditional logic in form of select expressions, traits which describe their
 * grammatical features, and can use Fluent builtins which make use of the
 * `Intl` formatters to format numbers, dates, lists and more into the
 * bundle's language. See the documentation of the Fluent syntax for more
 * information.
 */
class FluentBundle {

  /**
   * Create an instance of `FluentBundle`.
   *
   * The `locales` argument is used to instantiate `Intl` formatters used by
   * translations.  The `options` object can be used to configure the bundle.
   *
   * Examples:
   *
   *     const bundle = new FluentBundle(locales);
   *
   *     const bundle = new FluentBundle(locales, { useIsolating: false });
   *
   *     const bundle = new FluentBundle(locales, {
   *       useIsolating: true,
   *       functions: {
   *         NODE_ENV: () => process.env.NODE_ENV
   *       }
   *     });
   *
   * Available options:
   *
   *   - `functions` - an object of additional functions available to
   *                   translations as builtins.
   *
   *   - `useIsolating` - boolean specifying whether to use Unicode isolation
   *                    marks (FSI, PDI) for bidi interpolations.
   *
   *   - `transform` - a function used to transform string parts of patterns.
   *
   * @param   {string|Array<string>} locales - Locale or locales of the bundle
   * @param   {Object} [options]
   * @returns {FluentBundle}
   */
  constructor(locales, {
    functions = {},
    useIsolating = true,
    transform = v => v
  } = {}) {
    this.locales = Array.isArray(locales) ? locales : [locales];

    this._terms = new Map();
    this._messages = new Map();
    this._functions = functions;
    this._useIsolating = useIsolating;
    this._transform = transform;
    this._intls = new WeakMap();
  }

  /*
   * Return an iterator over public `[id, message]` pairs.
   *
   * @returns {Iterator}
   */
  get messages() {
    return this._messages[Symbol.iterator]();
  }

  /*
   * Check if a message is present in the bundle.
   *
   * @param {string} id - The identifier of the message to check.
   * @returns {bool}
   */
  hasMessage(id) {
    return this._messages.has(id);
  }

  /*
   * Return the internal representation of a message.
   *
   * The internal representation should only be used as an argument to
   * `FluentBundle.format`.
   *
   * @param {string} id - The identifier of the message to check.
   * @returns {Any}
   */
  getMessage(id) {
    return this._messages.get(id);
  }

  /**
   * Add a translation resource to the bundle.
   *
   * The translation resource must use the Fluent syntax.  It will be parsed by
   * the bundle and each translation unit (message) will be available in the
   * bundle by its identifier.
   *
   *     bundle.addMessages('foo = Foo');
   *     bundle.getMessage('foo');
   *
   *     // Returns a raw representation of the 'foo' message.
   *
   * Parsed entities should be formatted with the `format` method in case they
   * contain logic (references, select expressions etc.).
   *
   * @param   {string} source - Text resource with translations.
   * @returns {Array<Error>}
   */
  addMessages(source) {
    const res = FluentResource.fromString(source);
    return this.addResource(res);
  }

  /**
   * Add a translation resource to the bundle.
   *
   * The translation resource must be an instance of FluentResource,
   * e.g. parsed by `FluentResource.fromString`.
   *
   *     let res = FluentResource.fromString("foo = Foo");
   *     bundle.addResource(res);
   *     bundle.getMessage('foo');
   *
   *     // Returns a raw representation of the 'foo' message.
   *
   * Parsed entities should be formatted with the `format` method in case they
   * contain logic (references, select expressions etc.).
   *
   * @param   {FluentResource} res - FluentResource object.
   * @returns {Array<Error>}
   */
  addResource(res) {
    const errors = res.errors.slice();
    for (const [id, value] of res) {
      if (id.startsWith("-")) {
        // Identifiers starting with a dash (-) define terms. Terms are private
        // and cannot be retrieved from FluentBundle.
        if (this._terms.has(id)) {
          errors.push(`Attempt to override an existing term: "${id}"`);
          continue;
        }
        this._terms.set(id, value);
      } else {
        if (this._messages.has(id)) {
          errors.push(`Attempt to override an existing message: "${id}"`);
          continue;
        }
        this._messages.set(id, value);
      }
    }

    return errors;
  }

  /**
   * Format a message to a string or null.
   *
   * Format a raw `message` from the bundle into a string (or a null if it has
   * a null value).  `args` will be used to resolve references to variables
   * passed as arguments to the translation.
   *
   * In case of errors `format` will try to salvage as much of the translation
   * as possible and will still return a string.  For performance reasons, the
   * encountered errors are not returned but instead are appended to the
   * `errors` array passed as the third argument.
   *
   *     const errors = [];
   *     bundle.addMessages('hello = Hello, { $name }!');
   *     const hello = bundle.getMessage('hello');
   *     bundle.format(hello, { name: 'Jane' }, errors);
   *
   *     // Returns 'Hello, Jane!' and `errors` is empty.
   *
   *     bundle.format(hello, undefined, errors);
   *
   *     // Returns 'Hello, name!' and `errors` is now:
   *
   *     [<ReferenceError: Unknown variable: name>]
   *
   * @param   {Object | string}    message
   * @param   {Object | undefined} args
   * @param   {Array}              errors
   * @returns {?string}
   */
  format(message, args, errors) {
    // optimize entities which are simple strings with no attributes
    if (typeof message === "string") {
      return this._transform(message);
    }

    // optimize simple-string entities with attributes
    if (typeof message.val === "string") {
      return this._transform(message.val);
    }

    // optimize entities with null values
    if (message.val === undefined) {
      return null;
    }

    return resolve(this, args, message, errors);
  }

  _memoizeIntlObject(ctor, opts) {
    const cache = this._intls.get(ctor) || {};
    const id = JSON.stringify(opts);

    if (!cache[id]) {
      cache[id] = new ctor(this.locales, opts);
      this._intls.set(ctor, cache);
    }

    return cache[id];
  }
}

/*
 * @module fluent
 * @overview
 *
 * `fluent` is a JavaScript implementation of Project Fluent, a localization
 * framework designed to unleash the expressive power of the natural language.
 *
 */

/* eslint no-console: ["error", {allow: ["warn"]}] */
/* global console */

// Match the opening angle bracket (<) in HTML tags, and HTML entities like
// &amp;, &#0038;, &#x0026;.
const reOverlay = /<|&#?\w+;/;

/**
 * Elements allowed in translations even if they are not present in the source
 * HTML. They are text-level elements as defined by the HTML5 spec:
 * https://www.w3.org/TR/html5/text-level-semantics.html with the exception of:
 *
 *   - a - because we don't allow href on it anyways,
 *   - ruby, rt, rp - because we don't allow nested elements to be inserted.
 */
const TEXT_LEVEL_ELEMENTS = {
  "http://www.w3.org/1999/xhtml": [
    "em", "strong", "small", "s", "cite", "q", "dfn", "abbr", "data",
    "time", "code", "var", "samp", "kbd", "sub", "sup", "i", "b", "u",
    "mark", "bdi", "bdo", "span", "br", "wbr"
  ],
};

const LOCALIZABLE_ATTRIBUTES = {
  "http://www.w3.org/1999/xhtml": {
    global: ["title", "aria-label", "aria-valuetext", "aria-moz-hint"],
    a: ["download"],
    area: ["download", "alt"],
    // value is special-cased in isAttrNameLocalizable
    input: ["alt", "placeholder"],
    menuitem: ["label"],
    menu: ["label"],
    optgroup: ["label"],
    option: ["label"],
    track: ["label"],
    img: ["alt"],
    textarea: ["placeholder"],
    th: ["abbr"]
  },
  "http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul": {
    global: [
      "accesskey", "aria-label", "aria-valuetext", "aria-moz-hint", "label"
    ],
    key: ["key", "keycode"],
    textbox: ["placeholder"],
    toolbarbutton: ["tooltiptext"],
  }
};


/**
 * Translate an element.
 *
 * Translate the element's text content and attributes. Some HTML markup is
 * allowed in the translation. The element's children with the data-l10n-name
 * attribute will be treated as arguments to the translation. If the
 * translation defines the same children, their attributes and text contents
 * will be used for translating the matching source child.
 *
 * @param   {Element} element
 * @param   {Object} translation
 * @private
 */
function translateElement(element, translation) {
  const {value} = translation;

  if (typeof value === "string") {
    if (!reOverlay.test(value)) {
      // If the translation doesn't contain any markup skip the overlay logic.
      element.textContent = value;
    } else {
      // Else parse the translation's HTML using an inert template element,
      // sanitize it and replace the element's content.
      const templateElement = element.ownerDocument.createElementNS(
        "http://www.w3.org/1999/xhtml", "template"
      );
      templateElement.innerHTML = value;
      overlayChildNodes(templateElement.content, element);
    }
  }

  // Even if the translation doesn't define any localizable attributes, run
  // overlayAttributes to remove any localizable attributes set by previous
  // translations.
  overlayAttributes(translation, element);
}

/**
 * Replace child nodes of an element with child nodes of another element.
 *
 * The contents of the target element will be cleared and fully replaced with
 * sanitized contents of the source element.
 *
 * @param {DocumentFragment} fromFragment - The source of children to overlay.
 * @param {Element} toElement - The target of the overlay.
 * @private
 */
function overlayChildNodes(fromFragment, toElement) {
  for (const childNode of fromFragment.childNodes) {
    if (childNode.nodeType === childNode.TEXT_NODE) {
      // Keep the translated text node.
      continue;
    }

    if (childNode.hasAttribute("data-l10n-name")) {
      const sanitized = namedChildFrom(toElement, childNode);
      fromFragment.replaceChild(sanitized, childNode);
      continue;
    }

    if (isElementAllowed(childNode)) {
      const sanitized = allowedChild(childNode);
      fromFragment.replaceChild(sanitized, childNode);
      continue;
    }

    console.warn(
      `An element of forbidden type "${childNode.localName}" was found in ` +
      "the translation. Only safe text-level elements and elements with " +
      "data-l10n-name are allowed."
    );

    // If all else fails, replace the element with its text content.
    fromFragment.replaceChild(textNode(childNode), childNode);
  }

  toElement.textContent = "";
  toElement.appendChild(fromFragment);
}

/**
 * Transplant localizable attributes of an element to another element.
 *
 * Any localizable attributes already set on the target element will be
 * cleared.
 *
 * @param   {Element|Object} fromElement - The source of child nodes to overlay.
 * @param   {Element} toElement - The target of the overlay.
 * @private
 */
function overlayAttributes(fromElement, toElement) {
  const explicitlyAllowed = toElement.hasAttribute("data-l10n-attrs")
    ? toElement.getAttribute("data-l10n-attrs")
      .split(",").map(i => i.trim())
    : null;

  // Remove existing localizable attributes.
  for (const attr of Array.from(toElement.attributes)) {
    if (isAttrNameLocalizable(attr.name, toElement, explicitlyAllowed)) {
      toElement.removeAttribute(attr.name);
    }
  }

  // fromElement might be a {value, attributes} object as returned by
  // Localization.messageFromContext. In which case attributes may be null to
  // save GC cycles.
  if (!fromElement.attributes) {
    return;
  }

  // Set localizable attributes.
  for (const attr of Array.from(fromElement.attributes)) {
    if (isAttrNameLocalizable(attr.name, toElement, explicitlyAllowed)) {
      toElement.setAttribute(attr.name, attr.value);
    }
  }
}

/**
 * Sanitize a child element created by the translation.
 *
 * Try to find a corresponding child in sourceElement and use it as the base
 * for the sanitization. This will preserve functional attribtues defined on
 * the child element in the source HTML.
 *
 * @param   {Element} sourceElement - The source for data-l10n-name lookups.
 * @param   {Element} translatedChild - The translated child to be sanitized.
 * @returns {Element}
 * @private
 */
function namedChildFrom(sourceElement, translatedChild) {
  const childName = translatedChild.getAttribute("data-l10n-name");
  const sourceChild = sourceElement.querySelector(
    `[data-l10n-name="${childName}"]`
  );

  if (!sourceChild) {
    console.warn(
      `An element named "${childName}" wasn't found in the source.`
    );
    return textNode(translatedChild);
  }

  if (sourceChild.localName !== translatedChild.localName) {
    console.warn(
      `An element named "${childName}" was found in the translation ` +
      `but its type ${translatedChild.localName} didn't match the ` +
      `element found in the source (${sourceChild.localName}).`
    );
    return textNode(translatedChild);
  }

  // Remove it from sourceElement so that the translation cannot use
  // the same reference name again.
  sourceElement.removeChild(sourceChild);
  // We can't currently guarantee that a translation won't remove
  // sourceChild from the element completely, which could break the app if
  // it relies on an event handler attached to the sourceChild. Let's make
  // this limitation explicit for now by breaking the identitiy of the
  // sourceChild by cloning it. This will destroy all event handlers
  // attached to sourceChild via addEventListener and via on<name>
  // properties.
  const clone = sourceChild.cloneNode(false);
  return shallowPopulateUsing(translatedChild, clone);
}

/**
 * Sanitize an allowed element.
 *
 * Text-level elements allowed in translations may only use safe attributes
 * and will have any nested markup stripped to text content.
 *
 * @param   {Element} element - The element to be sanitized.
 * @returns {Element}
 * @private
 */
function allowedChild(element) {
  // Start with an empty element of the same type to remove nested children
  // and non-localizable attributes defined by the translation.
  const clone = element.ownerDocument.createElement(element.localName);
  return shallowPopulateUsing(element, clone);
}

/**
 * Convert an element to a text node.
 *
 * @param   {Element} element - The element to be sanitized.
 * @returns {Node}
 * @private
 */
function textNode(element) {
  return element.ownerDocument.createTextNode(element.textContent);
}

/**
 * Check if element is allowed in the translation.
 *
 * This method is used by the sanitizer when the translation markup contains
 * an element which is not present in the source code.
 *
 * @param   {Element} element
 * @returns {boolean}
 * @private
 */
function isElementAllowed(element) {
  const allowed = TEXT_LEVEL_ELEMENTS[element.namespaceURI];
  return allowed && allowed.includes(element.localName);
}

/**
 * Check if attribute is allowed for the given element.
 *
 * This method is used by the sanitizer when the translation markup contains
 * DOM attributes, or when the translation has traits which map to DOM
 * attributes.
 *
 * `explicitlyAllowed` can be passed as a list of attributes explicitly
 * allowed on this element.
 *
 * @param   {string}         name
 * @param   {Element}        element
 * @param   {Array}          explicitlyAllowed
 * @returns {boolean}
 * @private
 */
function isAttrNameLocalizable(name, element, explicitlyAllowed = null) {
  if (explicitlyAllowed && explicitlyAllowed.includes(name)) {
    return true;
  }

  const allowed = LOCALIZABLE_ATTRIBUTES[element.namespaceURI];
  if (!allowed) {
    return false;
  }

  const attrName = name.toLowerCase();
  const elemName = element.localName;

  // Is it a globally safe attribute?
  if (allowed.global.includes(attrName)) {
    return true;
  }

  // Are there no allowed attributes for this element?
  if (!allowed[elemName]) {
    return false;
  }

  // Is it allowed on this element?
  if (allowed[elemName].includes(attrName)) {
    return true;
  }

  // Special case for value on HTML inputs with type button, reset, submit
  if (element.namespaceURI === "http://www.w3.org/1999/xhtml" &&
      elemName === "input" && attrName === "value") {
    const type = element.type.toLowerCase();
    if (type === "submit" || type === "button" || type === "reset") {
      return true;
    }
  }

  return false;
}

/**
 * Helper to set textContent and localizable attributes on an element.
 *
 * @param   {Element} fromElement
 * @param   {Element} toElement
 * @returns {Element}
 * @private
 */
function shallowPopulateUsing(fromElement, toElement) {
  toElement.textContent = fromElement.textContent;
  overlayAttributes(fromElement, toElement);
  return toElement;
}

/*
 * Base CachedIterable class.
 */
class CachedIterable extends Array {
    /**
     * Create a `CachedIterable` instance from an iterable or, if another
     * instance of `CachedIterable` is passed, return it without any
     * modifications.
     *
     * @param {Iterable} iterable
     * @returns {CachedIterable}
     */
    static from(iterable) {
        if (iterable instanceof this) {
            return iterable;
        }

        return new this(iterable);
    }
}

/*
 * CachedAsyncIterable caches the elements yielded by an async iterable.
 *
 * It can be used to iterate over an iterable many times without depleting the
 * iterable.
 */
class CachedAsyncIterable extends CachedIterable {
    /**
     * Create an `CachedAsyncIterable` instance.
     *
     * @param {Iterable} iterable
     * @returns {CachedAsyncIterable}
     */
    constructor(iterable) {
        super();

        if (Symbol.asyncIterator in Object(iterable)) {
            this.iterator = iterable[Symbol.asyncIterator]();
        } else if (Symbol.iterator in Object(iterable)) {
            this.iterator = iterable[Symbol.iterator]();
        } else {
            throw new TypeError("Argument must implement the iteration protocol.");
        }
    }

    /**
     * Synchronous iterator over the cached elements.
     *
     * Return a generator object implementing the iterator protocol over the
     * cached elements of the original (async or sync) iterable.
     */
    [Symbol.iterator]() {
        const cached = this;
        let cur = 0;

        return {
            next() {
                if (cached.length === cur) {
                    return {value: undefined, done: true};
                }
                return cached[cur++];
            }
        };
    }

    /**
     * Asynchronous iterator caching the yielded elements.
     *
     * Elements yielded by the original iterable will be cached and available
     * synchronously. Returns an async generator object implementing the
     * iterator protocol over the elements of the original (async or sync)
     * iterable.
     */
    [Symbol.asyncIterator]() {
        const cached = this;
        let cur = 0;

        return {
            async next() {
                if (cached.length <= cur) {
                    cached.push(await cached.iterator.next());
                }
                return cached[cur++];
            }
        };
    }

    /**
     * This method allows user to consume the next element from the iterator
     * into the cache.
     *
     * @param {number} count - number of elements to consume
     */
    async touchNext(count = 1) {
        let idx = 0;
        while (idx++ < count) {
            const last = this[this.length - 1];
            if (last && last.done) {
                break;
            }
            this.push(await this.iterator.next());
        }
        // Return the last cached {value, done} object to allow the calling
        // code to decide if it needs to call touchNext again.
        return this[this.length - 1];
    }
}

/* eslint no-console: ["error", { allow: ["warn", "error"] }] */

/**
 * The `Localization` class is a central high-level API for vanilla
 * JavaScript use of Fluent.
 * It combines language negotiation, MessageContext and I/O to
 * provide a scriptable API to format translations.
 */
class Localization {
  /**
   * @param {Array<String>} resourceIds      - List of resource IDs
   * @param {Function}      generateMessages - Function that returns a
   *                                           generator over MessageContexts
   *
   * @returns {Localization}
   */
  constructor(resourceIds = [], generateMessages) {
    this.resourceIds = resourceIds;
    this.generateMessages = generateMessages;
    this.ctxs = CachedAsyncIterable.from(
      this.generateMessages(this.resourceIds));
  }

  addResourceIds(resourceIds) {
    this.resourceIds.push(...resourceIds);
    this.onChange();
    return this.resourceIds.length;
  }

  removeResourceIds(resourceIds) {
    this.resourceIds = this.resourceIds.filter(r => !resourceIds.includes(r));
    this.onChange();
    return this.resourceIds.length;
  }

  /**
   * Format translations and handle fallback if needed.
   *
   * Format translations for `keys` from `MessageContext` instances on this
   * DOMLocalization. In case of errors, fetch the next context in the
   * fallback chain.
   *
   * @param   {Array<Object>}         keys    - Translation keys to format.
   * @param   {Function}              method  - Formatting function.
   * @returns {Promise<Array<string|Object>>}
   * @private
   */
  async formatWithFallback(keys, method) {
    const translations = [];

    for await (const ctx of this.ctxs) {
      const missingIds = keysFromContext(method, ctx, keys, translations);

      if (missingIds.size === 0) {
        break;
      }

      if (typeof console !== "undefined") {
        const locale = ctx.locales[0];
        const ids = Array.from(missingIds).join(", ");
        console.warn(`Missing translations in ${locale}: ${ids}`);
      }
    }

    return translations;
  }

  /**
   * Format translations into {value, attributes} objects.
   *
   * The fallback logic is the same as in `formatValues` but the argument type
   * is stricter (an array of arrays) and it returns {value, attributes}
   * objects which are suitable for the translation of DOM elements.
   *
   *     docL10n.formatMessages([
   *       {id: 'hello', args: { who: 'Mary' }},
   *       {id: 'welcome'}
   *     ]).then(console.log);
   *
   *     // [
   *     //   { value: 'Hello, Mary!', attributes: null },
   *     //   { value: 'Welcome!', attributes: { title: 'Hello' } }
   *     // ]
   *
   * Returns a Promise resolving to an array of the translation strings.
   *
   * @param   {Array<Object>} keys
   * @returns {Promise<Array<{value: string, attributes: Object}>>}
   * @private
   */
  formatMessages(keys) {
    return this.formatWithFallback(keys, messageFromContext);
  }

  /**
   * Retrieve translations corresponding to the passed keys.
   *
   * A generalized version of `DOMLocalization.formatValue`. Keys can
   * either be simple string identifiers or `[id, args]` arrays.
   *
   *     docL10n.formatValues([
   *       {id: 'hello', args: { who: 'Mary' }},
   *       {id: 'hello', args: { who: 'John' }},
   *       {id: 'welcome'}
   *     ]).then(console.log);
   *
   *     // ['Hello, Mary!', 'Hello, John!', 'Welcome!']
   *
   * Returns a Promise resolving to an array of the translation strings.
   *
   * @param   {Array<Object>} keys
   * @returns {Promise<Array<string>>}
   */
  formatValues(keys) {
    return this.formatWithFallback(keys, valueFromContext);
  }

  /**
   * Retrieve the translation corresponding to the `id` identifier.
   *
   * If passed, `args` is a simple hash object with a list of variables that
   * will be interpolated in the value of the translation.
   *
   *     docL10n.formatValue(
   *       'hello', { who: 'world' }
   *     ).then(console.log);
   *
   *     // 'Hello, world!'
   *
   * Returns a Promise resolving to the translation string.
   *
   * Use this sparingly for one-off messages which don't need to be
   * retranslated when the user changes their language preferences, e.g. in
   * notifications.
   *
   * @param   {string}  id     - Identifier of the translation to format
   * @param   {Object}  [args] - Optional external arguments
   * @returns {Promise<string>}
   */
  async formatValue(id, args) {
    const [val] = await this.formatValues([{id, args}]);
    return val;
  }

  handleEvent() {
    this.onChange();
  }

  /**
   * This method should be called when there's a reason to believe
   * that language negotiation or available resources changed.
   */
  onChange() {
    this.ctxs = CachedAsyncIterable.from(
      this.generateMessages(this.resourceIds));
    this.ctxs.touchNext(2);
  }
}

/**
 * Format the value of a message into a string.
 *
 * This function is passed as a method to `keysFromContext` and resolve
 * a value of a single L10n Entity using provided `MessageContext`.
 *
 * If the function fails to retrieve the entity, it will return an ID of it.
 * If formatting fails, it will return a partially resolved entity.
 *
 * In both cases, an error is being added to the errors array.
 *
 * @param   {MessageContext} ctx
 * @param   {Array<Error>}   errors
 * @param   {string}         id
 * @param   {Object}         args
 * @returns {string}
 * @private
 */
function valueFromContext(ctx, errors, id, args) {
  const msg = ctx.getMessage(id);
  return ctx.format(msg, args, errors);
}

/**
 * Format all public values of a message into a {value, attributes} object.
 *
 * This function is passed as a method to `keysFromContext` and resolve
 * a single L10n Entity using provided `MessageContext`.
 *
 * The function will return an object with a value and attributes of the
 * entity.
 *
 * If the function fails to retrieve the entity, the value is set to the ID of
 * an entity, and attributes to `null`. If formatting fails, it will return
 * a partially resolved value and attributes.
 *
 * In both cases, an error is being added to the errors array.
 *
 * @param   {MessageContext} ctx
 * @param   {Array<Error>}   errors
 * @param   {String}         id
 * @param   {Object}         args
 * @returns {Object}
 * @private
 */
function messageFromContext(ctx, errors, id, args) {
  const msg = ctx.getMessage(id);

  const formatted = {
    value: ctx.format(msg, args, errors),
    attributes: null,
  };

  if (msg.attrs) {
    formatted.attributes = [];
    for (const [name, attr] of Object.entries(msg.attrs)) {
      const value = ctx.format(attr, args, errors);
      if (value !== null) {
        formatted.attributes.push({name, value});
      }
    }
  }

  return formatted;
}

/**
 * This function is an inner function for `Localization.formatWithFallback`.
 *
 * It takes a `MessageContext`, list of l10n-ids and a method to be used for
 * key resolution (either `valueFromContext` or `messageFromContext`) and
 * optionally a value returned from `keysFromContext` executed against
 * another `MessageContext`.
 *
 * The idea here is that if the previous `MessageContext` did not resolve
 * all keys, we're calling this function with the next context to resolve
 * the remaining ones.
 *
 * In the function, we loop over `keys` and check if we have the `prev`
 * passed and if it has an error entry for the position we're in.
 *
 * If it doesn't, it means that we have a good translation for this key and
 * we return it. If it does, we'll try to resolve the key using the passed
 * `MessageContext`.
 *
 * In the end, we fill the translations array, and return the Set with
 * missing ids.
 *
 * See `Localization.formatWithFallback` for more info on how this is used.
 *
 * @param {Function}       method
 * @param {MessageContext} ctx
 * @param {Array<string>}  keys
 * @param {{Array<{value: string, attributes: Object}>}} translations
 *
 * @returns {Set<string>}
 * @private
 */
function keysFromContext(method, ctx, keys, translations) {
  const messageErrors = [];
  const missingIds = new Set();

  keys.forEach(({id, args}, i) => {
    if (translations[i] !== undefined) {
      return;
    }

    if (ctx.hasMessage(id)) {
      messageErrors.length = 0;
      translations[i] = method(ctx, messageErrors, id, args);
      // XXX: Report resolver errors
    } else {
      missingIds.add(id);
    }
  });

  return missingIds;
}

const L10NID_ATTR_NAME = "data-l10n-id";
const L10NARGS_ATTR_NAME = "data-l10n-args";

const L10N_ELEMENT_QUERY = `[${L10NID_ATTR_NAME}]`;

/**
 * The `DOMLocalization` class is responsible for fetching resources and
 * formatting translations.
 *
 * It implements the fallback strategy in case of errors encountered during the
 * formatting of translations and methods for observing DOM
 * trees with a `MutationObserver`.
 */
class DOMLocalization extends Localization {
  /**
   * @param {Array<String>}    resourceIds      - List of resource IDs
   * @param {Function}         generateMessages - Function that returns a
   *                                              generator over MessageContexts
   * @returns {DOMLocalization}
   */
  constructor(resourceIds, generateMessages) {
    super(resourceIds, generateMessages);

    // A Set of DOM trees observed by the `MutationObserver`.
    this.roots = new Set();
    // requestAnimationFrame handler.
    this.pendingrAF = null;
    // list of elements pending for translation.
    this.pendingElements = new Set();
    this.windowElement = null;
    this.mutationObserver = null;

    this.observerConfig = {
      attribute: true,
      characterData: false,
      childList: true,
      subtree: true,
      attributeFilter: [L10NID_ATTR_NAME, L10NARGS_ATTR_NAME]
    };
  }

  onChange() {
    super.onChange();
    this.translateRoots();
  }

  /**
   * Set the `data-l10n-id` and `data-l10n-args` attributes on DOM elements.
   * FluentDOM makes use of mutation observers to detect changes
   * to `data-l10n-*` attributes and translate elements asynchronously.
   * `setAttributes` is a convenience method which allows to translate
   * DOM elements declaratively.
   *
   * You should always prefer to use `data-l10n-id` on elements (statically in
   * HTML or dynamically via `setAttributes`) over manually retrieving
   * translations with `format`.  The use of attributes ensures that the
   * elements can be retranslated when the user changes their language
   * preferences.
   *
   * ```javascript
   * localization.setAttributes(
   *   document.querySelector('#welcome'), 'hello', { who: 'world' }
   * );
   * ```
   *
   * This will set the following attributes on the `#welcome` element.
   * The MutationObserver will pick up this change and will localize the element
   * asynchronously.
   *
   * ```html
   * <p id='welcome'
   *   data-l10n-id='hello'
   *   data-l10n-args='{"who": "world"}'>
   * </p>
   * ```
   *
   * @param {Element}                element - Element to set attributes on
   * @param {string}                 id      - l10n-id string
   * @param {Object<string, string>} args    - KVP list of l10n arguments
   * @returns {Element}
   */
  setAttributes(element, id, args) {
    element.setAttribute(L10NID_ATTR_NAME, id);
    if (args) {
      element.setAttribute(L10NARGS_ATTR_NAME, JSON.stringify(args));
    } else {
      element.removeAttribute(L10NARGS_ATTR_NAME);
    }
    return element;
  }

  /**
   * Get the `data-l10n-*` attributes from DOM elements.
   *
   * ```javascript
   * localization.getAttributes(
   *   document.querySelector('#welcome')
   * );
   * // -> { id: 'hello', args: { who: 'world' } }
   * ```
   *
   * @param   {Element}  element - HTML element
   * @returns {{id: string, args: Object}}
   */
  getAttributes(element) {
    return {
      id: element.getAttribute(L10NID_ATTR_NAME),
      args: JSON.parse(element.getAttribute(L10NARGS_ATTR_NAME) || null)
    };
  }

  /**
   * Add `newRoot` to the list of roots managed by this `DOMLocalization`.
   *
   * Additionally, if this `DOMLocalization` has an observer, start observing
   * `newRoot` in order to translate mutations in it.
   *
   * @param {Element}      newRoot - Root to observe.
   */
  connectRoot(newRoot) {
    for (const root of this.roots) {
      if (root === newRoot ||
          root.contains(newRoot) ||
          newRoot.contains(root)) {
        throw new Error("Cannot add a root that overlaps with existing root.");
      }
    }

    if (this.windowElement) {
      if (this.windowElement !== newRoot.ownerDocument.defaultView) {
        throw new Error(`Cannot connect a root:
          DOMLocalization already has a root from a different window.`);
      }
    } else {
      this.windowElement = newRoot.ownerDocument.defaultView;
      this.mutationObserver = new this.windowElement.MutationObserver(
        mutations => this.translateMutations(mutations)
      );
    }


    this.roots.add(newRoot);
    this.mutationObserver.observe(newRoot, this.observerConfig);
  }

  /**
   * Remove `root` from the list of roots managed by this `DOMLocalization`.
   *
   * Additionally, if this `DOMLocalization` has an observer, stop observing
   * `root`.
   *
   * Returns `true` if the root was the last one managed by this
   * `DOMLocalization`.
   *
   * @param   {Element} root - Root to disconnect.
   * @returns {boolean}
   */
  disconnectRoot(root) {
    this.roots.delete(root);
    // Pause the mutation observer to stop observing `root`.
    this.pauseObserving();

    if (this.roots.size === 0) {
      this.mutationObserver = null;
      this.windowElement = null;
      this.pendingrAF = null;
      this.pendingElements.clear();
      return true;
    }

    // Resume observing all other roots.
    this.resumeObserving();
    return false;
  }

  /**
   * Translate all roots associated with this `DOMLocalization`.
   *
   * @returns {Promise}
   */
  translateRoots() {
    const roots = Array.from(this.roots);
    return Promise.all(
      roots.map(root => this.translateFragment(root))
    );
  }

  /**
   * Pauses the `MutationObserver`.
   *
   * @private
   */
  pauseObserving() {
    if (!this.mutationObserver) {
      return;
    }

    this.translateMutations(this.mutationObserver.takeRecords());
    this.mutationObserver.disconnect();
  }

  /**
   * Resumes the `MutationObserver`.
   *
   * @private
   */
  resumeObserving() {
    if (!this.mutationObserver) {
      return;
    }

    for (const root of this.roots) {
      this.mutationObserver.observe(root, this.observerConfig);
    }
  }

  /**
   * Translate mutations detected by the `MutationObserver`.
   *
   * @private
   */
  translateMutations(mutations) {
    for (const mutation of mutations) {
      switch (mutation.type) {
        case "attributes":
          if (mutation.target.hasAttribute("data-l10n-id")) {
            this.pendingElements.add(mutation.target);
          }
          break;
        case "childList":
          for (const addedNode of mutation.addedNodes) {
            if (addedNode.nodeType === addedNode.ELEMENT_NODE) {
              if (addedNode.childElementCount) {
                for (const element of this.getTranslatables(addedNode)) {
                  this.pendingElements.add(element);
                }
              } else if (addedNode.hasAttribute(L10NID_ATTR_NAME)) {
                this.pendingElements.add(addedNode);
              }
            }
          }
          break;
      }
    }

    // This fragment allows us to coalesce all pending translations
    // into a single requestAnimationFrame.
    if (this.pendingElements.size > 0) {
      if (this.pendingrAF === null) {
        this.pendingrAF = this.windowElement.requestAnimationFrame(() => {
          this.translateElements(Array.from(this.pendingElements));
          this.pendingElements.clear();
          this.pendingrAF = null;
        });
      }
    }
  }

  /**
   * Translate a DOM element or fragment asynchronously using this
   * `DOMLocalization` object.
   *
   * Manually trigger the translation (or re-translation) of a DOM fragment.
   * Use the `data-l10n-id` and `data-l10n-args` attributes to mark up the DOM
   * with information about which translations to use.
   *
   * Returns a `Promise` that gets resolved once the translation is complete.
   *
   * @param   {DOMFragment} frag - Element or DocumentFragment to be translated
   * @returns {Promise}
   */
  translateFragment(frag) {
    return this.translateElements(this.getTranslatables(frag));
  }

  /**
   * Translate a list of DOM elements asynchronously using this
   * `DOMLocalization` object.
   *
   * Manually trigger the translation (or re-translation) of a list of elements.
   * Use the `data-l10n-id` and `data-l10n-args` attributes to mark up the DOM
   * with information about which translations to use.
   *
   * Returns a `Promise` that gets resolved once the translation is complete.
   *
   * @param   {Array<Element>} elements - List of elements to be translated
   * @returns {Promise}
   */
  async translateElements(elements) {
    if (!elements.length) {
      return undefined;
    }

    const keys = elements.map(this.getKeysForElement);
    const translations = await this.formatMessages(keys);
    return this.applyTranslations(elements, translations);
  }

  /**
   * Applies translations onto elements.
   *
   * @param {Array<Element>} elements
   * @param {Array<Object>}  translations
   * @private
   */
  applyTranslations(elements, translations) {
    this.pauseObserving();

    for (let i = 0; i < elements.length; i++) {
      if (translations[i] !== undefined) {
        translateElement(elements[i], translations[i]);
      }
    }

    this.resumeObserving();
  }

  /**
   * Collects all translatable child elements of the element.
   *
   * @param {Element} element
   * @returns {Array<Element>}
   * @private
   */
  getTranslatables(element) {
    const nodes = Array.from(element.querySelectorAll(L10N_ELEMENT_QUERY));

    if (typeof element.hasAttribute === "function" &&
        element.hasAttribute(L10NID_ATTR_NAME)) {
      nodes.push(element);
    }

    return nodes;
  }

  /**
   * Get the `data-l10n-*` attributes from DOM elements as a two-element
   * array.
   *
   * @param {Element} element
   * @returns {Object}
   * @private
   */
  getKeysForElement(element) {
    return {
      id: element.getAttribute(L10NID_ATTR_NAME),
      args: JSON.parse(element.getAttribute(L10NARGS_ATTR_NAME) || null)
    };
  }
}

/* eslint-env browser */

function documentReady() {
  const rs = document.readyState;
  if (rs === "interactive" || rs === "completed") {
    return Promise.resolve();
  }

  return new Promise(
    resolve => document.addEventListener(
      "readystatechange", resolve, { once: true }
    )
  );
}

function getMeta(elem) {
  return {
    available: elem.querySelector('meta[name="availableLanguages"]')
      .getAttribute("content")
      .split(",").map(s => s.trim()),
    default: elem.querySelector('meta[name="defaultLanguage"]')
      .getAttribute("content"),
  };
}

function getResourceLinks(elem) {
  return Array.prototype.map.call(
    elem.querySelectorAll('link[rel="localization"]'),
    el => el.getAttribute("href")
  );
}

async function fetchResource(locale, id) {
  const url = id.replace("{locale}", locale);
  const response = await fetch(url);
  return response.text();
}

async function createContext(locale, resourceIds) {
  const ctx = new FluentBundle([locale]);

  // First fetch all resources
  const resources = await Promise.all(
    resourceIds.map(id => fetchResource(locale, id))
  );

  // Then apply them preserving order
  for (const resource of resources) {
    ctx.addMessages(resource);
  }
  return ctx;
}

const meta = getMeta(document.head);

async function* generateMessages(resourceIds) {
  const locales = negotiateLanguages(
    navigator.languages,
    meta.available,
    {
      defaultLocale: meta.default
    }
  );
  for (const locale of locales) {
    yield await createContext(locale, resourceIds);
  }
}

const resourceIds = getResourceLinks(document.head);
document.l10n = new DOMLocalization(
  resourceIds, generateMessages
);
window.addEventListener("languagechange", document.l10n);

document.l10n.ready = documentReady().then(() => {
  document.l10n.connectRoot(document.documentElement);
  return document.l10n.translateRoots();
});
