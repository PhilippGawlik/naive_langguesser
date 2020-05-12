# Naive Language Guesser

Predict the language of a text by ngram-based probability distributions (models). Do so by generating some language models from language text example files with the ``model`` command. Subsequently apply the generated language models to an unclassified text artifact by using the ``guess`` command. The outcome will rank the existing language models from the most likely fit for the artifact in descending order.


## Todo

-  [x] add a leveled ngram model (current model is flat) 
-  [x] adjust smoothing functions accordingly
-  [x] move calculation of language probabilities to logspace
-  [x] ascii
-  [x] big corpora stress tests
-  [ ] parallel guessing stress test
-  [ ] utf-8 non-total model
-  [ ] count/probability models as matrices
-  [ ] refactoring, refactoring, refactoring

## Usage

### ``model`` Command

```
cargo run model [FLAGS] --alphabet <alphabet> --model-name <model_name> --n-gram-length <n_gram_length> --path <path> --smoothing-type <smoothing_type>

FLAGS:
	-h, --help          Prints help information
	-m, --set-marker    Specifies if marker '#' is added to start and end of the text
	-V, --version       Prints version information 

OPTIONS:
	-a, --alphabet <alphabet>                Specifies set of characters the language model is based on. Possible values: {alphanum, ascii}
	-n, --model-name <model_name>            Specifies name for generated model
	-l, --n-gram-length <n_gram_length>      Specifies the n-gram length the language model is based on
	-p, --path <path>                        Specifies the path to a text file holding a language example
	-s, --smoothing-type <smoothing_type>    Specify the type of smoothing. Possible values {no, add_one, witten_bell}
```

For more information about the flags/options see section **Modes**. The documentation can be found here: `cargo doc --open`.

### ``guess`` Command

```
cargo run guess [FLAGS] --alphabet <alphabet> --n-gram-length <n_gram_length> --path <path>

FLAGS:
	 -h, --help           Prints help information
	 -i, --in-parallel    Specifies parallel guessing over language models
	 -m, --set-marker     Specifies if marker '#' is added to start and end of the text
	 -V, --version        Prints version information

OPTIONS:
	-a, --alphabet <alphabet>              Specifies set of characters the language model is based on. Possible values {alphanum, ascii}
	-l, --n-gram-length <n_gram_length>    Specifies the n-gram length the language model is based on
	-p, --path <path>                      Specifies the path to a text file holding a language artifact
```
For more information about the flags/options see section **Modes**.

## Modes

**Naive Language Guesser** provides two modes of operation: **model** and **guess**.

The following aspects are relevant for both modes:

##### Alphabet

The alphabet concerns the set of symbols the language model is based upon. All symbols not included in the alphabet are ignored.

Currently the following alphabets are supported:

* `alphanum`: consists of lower/capital letters and numbers 
* `ascii`: consists of the set of ascii symbols (without control symbols; so 32-126)

##### NGram length

The ngram length specifies the length of the ngrams the language model is build upon and the language guessing is performed upon. We recommend `0 < n <= 3`.

##### Text Marker

If the ngram length is `1 < n` the information about being at the begin or end of a string `abc` would be lost, e.g. `n = 2` and the string being decomposed into `{ab, bc}`. If the flag `--set-marker` is set, a text marker marks the beginning and end of the string to save the information, e.g. textmarker `#` is added to `abc` as in `##abc##` to hold information about being at the begin or end as in `{##, #a, ab, bc, c#, ##}`.
   
### Model Mode

Generate a probability distribution model for a language example, based on ngrams to a certain length and an alphabet of symbols.

##### Smoothing

Smoothing is performed to deal with unseen ngrams. In case of unseen ngrams, a portion of the seen ngram counts is redistributed to the unseen ngram counts. By doing so, the language models are able to deal with unseen ngrams when applied to a text artifact.

Currently the followin smoothing techniques are provided:

* `no`: no smoothing is done
* `add_one`: add one to each seen/unseen ngram and normalise
* `witten_bell`: use the count of ngrams seen once to estimate the count of ngrams not seen

For more information on the smoothing techniques see:

*Speech and Language Processing
Daniel Jurafsky / James H. Martin
page 206: Smoothing
ISBN 0-13-095069-6*

### Guess Mode

Calculate the most likely language for a unclassified language artifact, based on a set of existing language models. The existing language models can be constructed with `model` command. The outcome will rank the existing language models from the most likely fit descending.

##### LOG space
The calculation is done in logspace to avoid vanishingly small probabilities. This might cast the probability scores to negative space. But because of monotony of the cast operation the ranking stays valid. 

##### Parallel processing
The calculation of the language models probabilities for a text artifact is done in parallel for all language models.

## Testing
For quick testing the repository includes three versions of the declaration of human rights (german, english and spanish) in `data/`, that can be used to build language models. 

For extensive testing we used the parallel corpus of EuroParl 

``Europarl: A Parallel Corpus for Statistical Machine Translation, Philipp Koehn, MT Summit 2005``

to perform a stress test up to 500 000 lines of text containing 170 million symbols on a system of:
```
RAM: 15,4 GiB
CPU: Intel® Core™ i7-8565U CPU @ 1.80GHz × 8
Disk: 928,0 GB
```

## Contribution

I like to express my gratitude for the contribution of Jean VanCoppenolle to this project in terms of string encoding, process optimisation and Rust-language specific coding advices. I added notes in the source code where his contributions are applied.
Thx Jean! :-)