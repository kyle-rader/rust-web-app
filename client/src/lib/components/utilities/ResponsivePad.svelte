<p></p>

<style lang="less">
	@import '../../../css/responsive.less';

	p {
		@media (max-width: @mobile) {
			margin-top: 10px;
		}

		@media (min-width: @tablet-min) and (max-width: @tablet-max) {
			margin-top: 30px;
		}

		@media (min-width: @desktop) {
			margin-top: 55px;
		}
	}
</style>
